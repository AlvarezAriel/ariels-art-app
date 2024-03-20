use octotablet::events::{PadEvent, TabletEvent, ToolEvent};
use octotablet::pad;
use octotablet::tablet::Tablet;
use octotablet::tool::Tool;
use crate::framework::main_loop;

pub struct Application {}

impl Application {
    pub fn new() -> Self {
        Application {}
    }

    pub fn configure<T>(
        &mut self,
        model: ModelFn<T>,
        update: UpdateFn<T>,
        render: RenderFn<T>,
        event: EventFn<T>,
    ) -> Runtime<T> {
        Runtime {
            application: self,
            model: model(self),
            update,
            render,
            event,
        }
    }
}

pub struct Runtime<'a, Model> {
    application: &'a Application,
    model: Model,
    update: UpdateFn<Model>,
    render: RenderFn<Model>,
    event: EventFn<Model>,
}

#[derive(Clone, Copy, Debug)]
pub enum RuntimeEvent<'a> {
    Stylus {
        event: octotablet::events::Event<'a>,
    },
}

impl<Model> Runtime<'_, Model> {
    pub fn run(&mut self) {
        let rt = Self::build_runtime();
        rt.block_on(main_loop::run(self))
    }

    pub fn publish_tablet_event(&mut self, event: octotablet::events::Event) {
        (self.event)(self.application, &mut self.model);
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

pub type EventFn<Model> = fn(&Application, &mut Model);