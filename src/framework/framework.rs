use crate::framework::main_loop;

pub struct Application {}

impl Application {
    pub fn new() -> Self {
        Application {}
    }

    pub fn configure<T>(&mut self, model_builder: ModelFn<T>) -> Runtime<T> {
        Runtime {
            model: model_builder(self)
        }
    }
}

pub struct Runtime<Model> {
    model: Model,
}

impl<Model> Runtime<Model> {
    pub fn run(&mut self) {
        let rt = Self::build_runtime();
        rt.block_on(main_loop::run())
    }

    fn build_runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("failed to create tokio runtime")
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Update {
    pub delta: std::time::Duration,
}

pub type ModelFn<Model> = fn(&Application) -> Model;

pub type UpdateFn<Model> = fn(&Application, &mut Model, Update);

pub type RenderFn<Model> = fn(&Application, &mut Model);

pub type EventFn<Model, Event> = fn(&Application, &mut Model, Event);