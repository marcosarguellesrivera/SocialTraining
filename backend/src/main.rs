use axum::{routing::get, Router};
use tokio::net::TcpListener;

async fn hello_world() -> &'static str {
    "Welcome to SocialTraining!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Servidor escuchando en http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
