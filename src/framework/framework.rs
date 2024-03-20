pub struct App {

}

impl<Model, Event> App {
    pub fn new() -> Self {
        App {

        }
    }

    pub fn run() {

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


