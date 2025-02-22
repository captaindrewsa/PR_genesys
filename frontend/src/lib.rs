mod routes;
use axum::routing::get;
use routes::create_routes;



pub async fn run_app(){
    let app = create_routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}