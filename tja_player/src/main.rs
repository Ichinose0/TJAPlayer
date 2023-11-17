use glge::GLGEApplication;

pub struct TJAPlayer {

}

impl GLGEApplication for TJAPlayer {
    fn init(&mut self,app: &mut glge::Application) {
        app.title = "太鼓の達人 for PC".to_owned();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    tja::TJA::load(&args[1]);
    // let app = TJAPlayer {};
    // glge::run_app(app);
}
