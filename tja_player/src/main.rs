#[macro_use]
extern crate log;

use glge::{context::Context, Frame, GLGEApplication};
use tja::TJA;

pub struct TJAComponent {
    tja: TJA,
}

pub struct TJAPlayer {
    component: TJAComponent,
    frame: GameFrame,
}

impl TJAPlayer {
    pub fn new(path: &str) -> Self {
        let tja = match TJA::load(path) {
            Ok(t) => t,
            Err(_) => {
                error!("Failed to load {}", path);
                warn!("Exit the application.");
                std::process::exit(-1);
            }
        };

        let frame = GameFrame {};
        Self {
            component: TJAComponent { tja },
            frame,
        }
    }
}

pub struct GameFrame {}

impl Frame for GameFrame {
    fn render(&self, ctx: &Context) {
        unsafe {
            gl::ClearColor(1.0, 0.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            ctx.swap();
        }
    }
}

pub struct MusicFrame {}

impl Frame for MusicFrame {
    fn render(&self, ctx: &Context) {
        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            ctx.swap();
        }
    }
}

impl GLGEApplication for TJAPlayer {
    fn init(&mut self, app: &mut glge::Application) {
        app.title = self.component.tja.get_title().to_owned();
    }

    fn render(&mut self, event: &glge::EventManager) {
        event.register_all(&[&self.frame]);
    }
}

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let args: Vec<String> = std::env::args().collect();

    let app = TJAPlayer::new(&args[1]);
    glge::run_app(app);
}
