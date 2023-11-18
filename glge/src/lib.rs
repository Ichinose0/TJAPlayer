pub(crate) mod context;
pub(crate) mod wgl;
pub(crate) mod gl;

use windows_sys::Win32::Graphics::OpenGL::{GL_COLOR_BUFFER_BIT, glClear, glClearColor};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use crate::context::Context;

pub struct Application {
    pub title: String,
    pub width: u32,
    pub height: u32
}

impl Default for Application {
    fn default() -> Self {
        Self {
            title: "GLGE".to_owned(),
            width: 800,
            height: 600
        }
    }
}

pub trait GLGEApplication {
    fn init(&mut self,app: &mut Application) {

    }
    
    fn event_loop(&mut self) {

    }
}

pub trait RenderObject {
    fn render();
}

pub fn run_app<T>(mut app: T) 
where
    T: GLGEApplication
{
    let mut app_config = Application::default();
    app.init(&mut app_config);
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title(app_config.title)
        .with_inner_size(winit::dpi::LogicalSize::new(800.0,600.0))
        .build(&event_loop)
        .unwrap();

    let ctx = Context::init(&window);

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::RedrawRequested => {
                    // Notify the windowing system that we'll be presenting to the window.
                    window.pre_present_notify();
                    unsafe {
                        glClearColor(1.0,0.0,0.0,1.0);
                        glClear(GL_COLOR_BUFFER_BIT);
                        ctx.swap();
                    }
                }
                _ => (),
            },
            Event::AboutToWait => {
                window.request_redraw();
            }

            _ => (),
        }
    });
    app.event_loop();
}