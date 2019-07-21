use glium::backend::glutin::Display;
use std::rc::Rc;

use imgui_glium_renderer::Renderer;

pub struct Gui {
    renderer: Renderer,
    context: imgui::Context,
    window: Rc<Display>, // this is shared with chaotic.rs
}


impl Gui {

    pub fn new(display: Rc<Display>) -> Self {

        let mut imgui = imgui::Context::create();

        imgui.set_ini_filename(None);

        let imgui_renderer =
            Renderer::init(&mut imgui, &*display.clone()).expect("Failed to initialize renderer");

        Gui {
            renderer: imgui_renderer,
            context: imgui,
            window: display,
        }

    }

    pub fn render(&mut self) {
        let gl_window = self.window.gl_window();
        let window_actual = gl_window.window();
        let pixel_dimensions = &window_actual.get_inner_size().unwrap();
        let hidpi_scaling = &window_actual.get_hidpi_factor(); // needed because modern screens are fucking wild.

        // you are, fighting brazen bullshit trying to get a dumb UI context to play
        //let ui = self.context.frame();
        //let window = (*self.window).gl_window();
    }


    // should return a context for
    // expressions to be evaluated in
    // to calculate the current position and
    // direction.
    // pub fn get_expressions() {

    //     unimplemented!();
    // }
}