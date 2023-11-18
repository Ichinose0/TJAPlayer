#[macro_use]
extern crate log;

pub mod context;
pub(crate) mod gl;
pub(crate) mod wgl;

use crate::context::Context;
use windows_sys::Win32::Graphics::OpenGL::{glClear, glClearColor, GL_COLOR_BUFFER_BIT};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

pub struct EventManager {
    render: Renderer,
}

impl EventManager {
    pub fn register_all(&self, frames: &[&impl Frame]) {
        for i in frames {
            self.render.render(i);
        }
    }
}

pub struct Renderer {
    ctx: Context,
}

impl Renderer {
    pub fn render(&self, frame: &&impl Frame) {
        frame.render(&self.ctx);
    }
}

#[derive(Debug)]
pub struct Application {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            title: "GLGE".to_owned(),
            width: 800,
            height: 600,
        }
    }
}

pub trait GLGEApplication {
    fn init(&mut self, app: &mut Application) {}

    fn render(&mut self, event: &EventManager) {}
}

pub trait Frame {
    fn render(&self, ctx: &Context);
}

pub fn run_app<T>(mut app: T)
where
    T: GLGEApplication,
{
    let mut app_config = Application::default();
    app.init(&mut app_config);
    info!("App configuration:\n{:#?}", app_config);

    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title(app_config.title)
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
        .build(&event_loop)
        .unwrap();

    let ctx = Context::init(&window);
    let event_manager = EventManager {
        render: Renderer { ctx },
    };

    let event_ref = &event_manager;

    match event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::RedrawRequested => {
                    // Notify the windowing system that we'll be presenting to the window.
                    window.pre_present_notify();
                    app.render(event_ref);
                }
                _ => (),
            },
            Event::AboutToWait => {
                window.request_redraw();
            }

            _ => (),
        }
    }) {
        Ok(_) => {}
        Err(e) => panic!("{:#?}", e),
    }
}
