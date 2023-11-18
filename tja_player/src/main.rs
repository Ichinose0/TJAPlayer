use glge::{GLGEApplication, Frame, context::Context};
use tja::TJA;

pub struct TJAComponent {
    tja: TJA
}

pub struct TJAPlayer {
    component: TJAComponent,
    frame: GameFrame
}

impl TJAPlayer {
    pub fn new(path: &str) -> Self {
        let tja = TJA::load(path).unwrap();
        let frame = GameFrame {};
        Self {
            component: TJAComponent {
                tja
            },
            frame
        }
    }
}

pub struct GameFrame {

}

impl Frame for GameFrame {
    fn render(&self,ctx: &Context) {
        unsafe {
            gl::ClearColor(1.0,0.0,1.0,1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            ctx.swap();
        }
    }
}

impl GLGEApplication for TJAPlayer {
    fn init(&mut self,app: &mut glge::Application) {
        app.title = self.component.tja.get_title().to_owned();
    }

    fn render(&mut self,event: &glge::EventManager) {
        event.register_all(&[&self.frame]);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let app = TJAPlayer::new(&args[1]);
    glge::run_app(app);
}
