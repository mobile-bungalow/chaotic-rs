use glium::backend::glutin::Display;
use std::rc::Rc;

#[macro_use]
use imgui::*;

use imgui::Ui;

use crate::chaotic::DynamicSystem;
use imgui_glium_renderer::Renderer;

use glutin::Event;

use std::time::Instant;

// As a note to every other person who is looking at self
// This package framework no longer supports glutin outside of
// the winit framework. which is good, but not documented.
// so use this crate instead of imgui_glutin_support
use imgui_winit_support::{HiDpiMode, WinitPlatform};

pub struct Gui {
    renderer: Renderer,
    context: imgui::Context,
    window: Rc<Display>, // this is shared with chaotic.rs
    last_frame: std::time::Instant,
    platform: WinitPlatform,
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

        Gui {
            renderer: imgui_renderer,
            context: imgui_c,
            window: display,
            last_frame: Instant::now(),
            platform: platform,
        }

    }
    // renders the UI, needs the dynamic system to display facts about it.
    pub fn render(&mut self, f: &mut glium::Frame, dy_sys: &DynamicSystem) {

        self.platform
            .prepare_frame(self.context.io_mut(), &*self.window.gl_window().window()) // step 4
            .expect("Failed to prepare frame");

        self.last_frame = self.context.io_mut().update_delta_time(self.last_frame);

        let ui: Ui = self.context.frame();
        //ui feature definitions
        ui.window(im_str!("User Configuration"))
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(|| {
                ui.separator();
            });

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

    // should return a context for
    // expressions to be evaluated in
    // to calculate the current position and
    // direction.
    // pub fn get_expressions() {

    //     unimplemented!();
    // }
}