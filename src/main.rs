
#[macro_use]
use glium::Display;


//use glium::glutin::dpi::LogicalPosition;
use glium::glutin::WindowEvent::*;

use glium::vertex::MultiVerticesSource;
use glium::*;
use glutin::{ContextBuilder, EventsLoop, WindowBuilder};
#[macro_use]
//use imgui::ImGui;
use imgui_glium_renderer::Renderer;


use std::thread::sleep_ms;
use std::time::{Duration, Instant};

mod chaotic;
mod gui;

use chaotic::DynamicSystem;
//use gui::Gui;

static VS: &str = include_str!("../shaders/chaos.vert.glsl");
static FS: &str = include_str!("../shaders/chaos.frag.glsl");

fn main() -> Result<(), Box<std::error::Error>> {
    let mut events_loop = EventsLoop::new();

    let display = Display::new(WindowBuilder::new(), ContextBuilder::new(), &events_loop).unwrap();


    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);

    let _imgui_renderer =
        Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    let program = glium::Program::from_source(&display, VS, FS, None)?;

    let mut exit = false;
    let _frame_delta = 0.0;

    let mut ds = DynamicSystem::new(display);

    let uniforms = uniform! {
        matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ]
    };

    let mut last_frame = Instant::now();

    loop {

        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    CloseRequested => exit = true,
                    _ => (),
                }
            }
        });


        let mut surface = ds.display.draw();

        surface.clear_color(0.0, 0.0, 0.0, 1.0);

        ds.update_vertex_buffer();
        // type annotation hell
        let vs: glium::vertex::VerticesSource = ds.get_vertices().into();
        surface
            .draw(
                vs,
                &ds.get_indices(),
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();

        surface.finish()?;

        let now = Instant::now();
        let delta = now - last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;

        if delta_s < 1.0 / 60.0 {
            std::thread::sleep(std::time::Duration::from_millis(
                (1000.0 * (1.0 / 60.0)) as u64,
            ));
        }

        if exit {
            break;
        }
    }
    Ok(())
}

