use crate::framework::framework;
use crate::framework::framework::{Application, Update};

struct Model {

}


pub fn run_app() {
    framework::Application::new().configure(
        model,
        update,
        render,
        event,
    ).run();
}

fn model(app: &Application) -> Model {
    Model{

    }
}

fn update(app: &Application,  _model: &mut Model, _update: Update) {

}

fn render(app: &Application,  _model: &mut Model) {

}

fn event(app: &Application,  _model: &mut Model) {

}