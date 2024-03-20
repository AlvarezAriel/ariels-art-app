mod app;

#[tokio::main]
async fn main() {
    app::app::run_app().await;
}