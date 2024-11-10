use frontend::run_app;

mod routes;

#[tokio::main]
async fn main() {
    run_app().await;
}
