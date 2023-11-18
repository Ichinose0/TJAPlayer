use glge::GLGEApplication;
use tja::TJA;

pub struct TJAPlayer {
    tja: TJA
}

impl TJAPlayer {
    pub fn new(path: &str) -> Self {
        let tja = TJA::load(path).unwrap();
        Self {
            tja
        }
    }
}

impl GLGEApplication for TJAPlayer {
    fn init(&mut self,app: &mut glge::Application) {
        app.title = self.tja.get_title().to_owned();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let app = TJAPlayer::new(&args[1]);
    glge::run_app(app);
}
