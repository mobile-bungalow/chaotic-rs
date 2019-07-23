// TODO : clean up imports

use glium::backend::glutin::Display;
use std::rc::Rc;

#[macro_use]
use imgui::*;

use crate::chaotic::{DynamicSystem, ProgramState};
use imgui_glium_renderer::Renderer;

use glutin::Event;
use std::time::Instant;

// the winit framework. which is good, but not documented.
// so use this crate instead of imgui_glutin_support
use imgui_winit_support::{HiDpiMode, WinitPlatform};

pub struct StringCollection {
    pub dx: ImString,                           // dx/dt
    pub dy: ImString,                           // dy/dt
    pub dz: ImString,                           // dz/dt
    pub fn_name_buffer: ImString, //holds function names before they are pushed onto expression vec
    pub expressions: Vec<(ImString, ImString)>, // function content, function name
}

enum GuiState {
    Normal,
    Error,
}

pub struct Gui {
    renderer: Renderer,
    context: imgui::Context,
    window: Rc<Display>, // this is shared with chaotic.rs
    last_frame: std::time::Instant,
    platform: WinitPlatform,
    math_strings: StringCollection,
}

impl Gui {
    pub fn new(display: Rc<Display>) -> Self {
        let mut imgui_c = imgui::Context::create();

        imgui_c.set_ini_filename(None);

        let imgui_renderer = Renderer::init(&mut imgui_c, &*display).unwrap();

        let mut platform = WinitPlatform::init(&mut imgui_c);
        {
            let gl_window = display.gl_window();
            let window = gl_window.window();
            platform.attach_window(imgui_c.io_mut(), &window, HiDpiMode::Rounded);
        }

        let map = StringCollection {
            dx: ImString::new("enter expression here."),
            dy: ImString::new("enter expression here."),
            dz: ImString::new("enter expression here."),
            fn_name_buffer: ImString::new("enter function name"),
            expressions: Vec::new(),
        };

        Gui {
            renderer: imgui_renderer,
            context: imgui_c,
            window: display,
            last_frame: Instant::now(),
            platform,
            math_strings: map,
        }
    }
    // 🍖 & 🥔
    // renders the UI.
    pub fn render(&mut self, f: &mut glium::Frame, dy_sys: &mut DynamicSystem) {
        // pre rendering stuff from the boilerplate
        self.platform
            .prepare_frame(self.context.io_mut(), &*self.window.gl_window().window())
            .expect("Failed to prepare frame");

        self.last_frame = self.context.io_mut().update_delta_time(self.last_frame);

        let ui = self.context.frame();

        // ui feature definitions these are done outside of closure
        // so that the closure does not have to borrow self, also clean code
        let dx_input = ui.input_text(im_str!("dx/dt"), &mut self.math_strings.dx);
        let dy_input = ui.input_text(im_str!("dy/dt"), &mut self.math_strings.dy);
        let dz_input = ui.input_text(im_str!("dz/dt"), &mut self.math_strings.dz);
        let fn_name = ui.input_text(im_str!(""), &mut self.math_strings.fn_name_buffer);

        // vector of tuples of the form: (expression holding c-string, fn label, index)
        let string_label_pairs: Vec<_> = self
            .math_strings
            .expressions
            .iter_mut()
            .enumerate()
            .map(|(i, (string, label))| (string, label, i))
            .collect();

        // kill me vec contains a byte string indicating which of the
        // expression fields to delete
        let mut kill_me_vec = vec![false; string_label_pairs.len()];
        let mut spawn_expr_field = false;

        // this tuple controls program flow
        // can't use dy sys state in the closure
        let mut start_stop_reset = (false, false, false);

        ui.window(im_str!("User Configuration"))
            .size([350.0, 190.0], Condition::FirstUseEver)
            .build(|| {
                // the add expression button and fields
                spawn_expr_field = ui.button(im_str!("add expression"), [150.0, 20.0]);
                let dims = ui.get_item_rect_size(); // gets LAST draw size
                ui.same_line(dims[0] + 20.0);
                fn_name.build();

                // --- dynamically draws the new function buttons as spawned by the user
                for (return_string, label, index) in string_label_pairs {
                    let button_title = ImString::from(format!("X##{}", index));
                    // the function field itself
                    let field = ui.input_text(label.as_ref(), return_string);
                    field.build();
                    let dims = ui.get_item_rect_size();
                    ui.same_line(dims[0] + 20.0);
                    // destruction button
                    kill_me_vec[index] = ui.button(button_title.as_ref(), [20.0, 20.0]);
                }
                // --- partial derivative input formulas
                dx_input.build();
                dy_input.build();
                dz_input.build();
                // --- white space and bar before bottom control flow buttons
                ui.spacing();
                ui.separator();
                ui.spacing();
                // --- bottom three control flow buttons
                ui.group(|| {
                    start_stop_reset.0 = ui.button(im_str!("Start"), [80.0, 30.0]);
                    ui.same_line(90.0);
                    start_stop_reset.1 = ui.button(im_str!("Stop"), [80.0, 30.0]);
                    ui.same_line(180.0);
                    start_stop_reset.2 = ui.button(im_str!("Reset"), [80.0, 30.0]);
                });
            });

        match (&dy_sys.state, start_stop_reset) {
            // these three cases prevent redundant state changes
            (ProgramState::Start, (true, _, _)) => {}
            (ProgramState::Stopped, (_, true, _)) => {}
            // these cases actually call their respective functions
            (_, (true, _, _)) => {
                if let Err(e) = dy_sys.resolve_system(&self.math_strings) {
                    //TODO: Display errors somehow I'm lazy.
                    println!("{}", e);
                } else {
                    dy_sys.start();
                }
            }
            (_, (_, true, _)) => dy_sys.stop(),
            (_, (_, _, true)) => dy_sys.reset(),
            _ => {}
        };

        // kill all functions requested to be killed this cycle
        for (index, killed) in kill_me_vec.iter().cloned().enumerate() {
            if killed {
                self.math_strings.expressions.remove(index);
            }
        }

        if spawn_expr_field {
            self.math_strings.expressions.push((
                ImString::new("Enter Expression Here"),
                self.math_strings.fn_name_buffer.clone(),
            ));
            self.math_strings.fn_name_buffer = ImString::new("enter function name");
        }

        // collect ui data
        let draw_data = ui.render();

        self.renderer
            .render(f, draw_data)
            .expect("could not render glium.");
    }

    // forwards all events to imGui
    pub fn handle_events(&mut self, event: &Event) {
        self.platform.handle_event(
            self.context.io_mut(),
            &*self.window.gl_window().window(),
            &event,
        );
    }
}
