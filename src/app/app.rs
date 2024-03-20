use crate::framework::framework;

pub async fn run_app() {
    framework::App::new().run().await;
}