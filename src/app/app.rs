use crate::framework::framework;
use crate::framework::framework::Application;

pub fn run_app() {
    framework::Application::new().configure(model).run();
}

struct Model {

}

fn model(app: &Application) -> Model {
    Model{

    }
}