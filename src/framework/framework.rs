use crate::framework::main_loop::run;

pub struct App {

}

impl App {
    pub fn new() -> Self {
        App {

        }
    }

    pub async fn run(&mut self) {
        run().await
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Update {
    pub delta: std::time::Duration,
}

pub type ModelFn<Model> = fn(&App) -> Model;

pub type UpdateFn<Model> = fn(&App, &mut Model, Update);

pub type RenderFn<Model> = fn(&App, &mut Model);

pub type EventFn<Model, Event> = fn(&App, &mut Model, Event);


