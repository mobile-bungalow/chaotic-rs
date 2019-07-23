use glium::backend::glutin::Display;
use std::rc::Rc;

#[macro_use]
use imgui::*;

use imgui::Ui;

use crate::chaotic::DynamicSystem;
use imgui_glium_renderer::Renderer;

use glutin::Event;

use std::collections::HashMap;

use std::time::Instant;
// the winit framework. which is good, but not documented.
// so use this crate instead of imgui_glutin_support
use imgui_winit_support::{HiDpiMode, WinitPlatform};

struct StringCollection {
    pub dx: ImString, // dx/dt
    pub dy: ImString, // dy/dt
    pub dz: ImString, // dz/dt
    pub expressions: Vec<ImString>,
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
            expressions: Vec::new(),
        };

        Gui {
            renderer: imgui_renderer,
            context: imgui_c,
            window: display,
            last_frame: Instant::now(),
            platform: platform,
            math_strings: map,

        }

    }
    // renders the UI, needs the dynamic system to display facts about it.
    pub fn render(&mut self, f: &mut glium::Frame, _dy_sys: &DynamicSystem) {

        self.platform
            .prepare_frame(self.context.io_mut(), &*self.window.gl_window().window()) // step 4
            .expect("Failed to prepare frame");

        self.last_frame = self.context.io_mut().update_delta_time(self.last_frame);

        let ui = self.context.frame();
        //ui feature definitions

        let it = ui.input_text(im_str!("dx/dt"), &mut self.math_strings.dx);
        let it1 = ui.input_text(im_str!("dy/dt"), &mut self.math_strings.dy);
        let it2 = ui.input_text(im_str!("dz/dt"), &mut self.math_strings.dz);

        let mut string_ref = Vec::new();
        self.math_strings
            .expressions
            .iter_mut()
            .enumerate()
            .for_each(|(i, x)| {
                // push tuple of index and ui element
                string_ref.push((x, i));
            });

        let mut kill_me_vec = vec![false; string_ref.len()];
        let mut push = false;

        let mut start_stop_reset = (false, false, false); // wouldn't it be cool if this optimized into a bitstring!!

        ui.window(im_str!("User Configuration"))
            .size([300.0, 190.0], Condition::FirstUseEver)
            .build(|| {

                push = ui.button(im_str!("add expression"), [150.0, 20.0]);

                for text_field in string_ref {
                    let string = ImString::from(format!("fn ${}", text_field.1)); // todo, make these nameable
                    let bt = ImString::from(format!("X##{}", text_field.1)); // todo, make these nameable
                    let field = ui.input_text(string.as_ref(), text_field.0);
                    field.build();
                    let dims = ui.get_item_rect_size(); // gets LAST draw size
                    ui.same_line(dims[0] + 20.0);
                    kill_me_vec[text_field.1] = ui.button(bt.as_ref(), [20.0, 20.0]);
                }

                it2.build();
                it1.build();
                it.build();
                ui.spacing();
                ui.separator();
                ui.spacing();


                ui.group(|| {
                    start_stop_reset.0 = ui.button(im_str!("Start"), [80.0, 30.0]);
                    ui.same_line(90.0);
                    start_stop_reset.1 = ui.button(im_str!("Stop"), [80.0, 30.0]);
                    ui.same_line(180.0);
                    start_stop_reset.2 = ui.button(im_str!("Reset"), [80.0, 30.0]);
                });
            });

        // can't use iterators because of borrowing
        for (index, killed) in kill_me_vec.iter().cloned().enumerate() {
            if killed {
                self.math_strings.expressions.remove(index);
            }
        }

        if push {
            &self
                .math_strings
                .expressions
                .push(ImString::new("Enter Expression Here"));
        }
        let draw_data = ui.render();

        self.renderer
            .render(f, draw_data)
            .expect("could not render glium.");
    }

    // forwards all events to IMGUI
    pub fn handle_events(&mut self, event: &Event) {
        self.platform.handle_event(
            self.context.io_mut(),
            &*self.window.gl_window().window(),
            &event,
        );
    }

}