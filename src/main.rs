mod app;
mod framework;

#[tokio::main]
async fn main() {
    app::app::run_app().await;
}