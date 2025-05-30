use axum::{routing::get, Router};
use tower_http::cors::{CorsLayer, Any};
use tokio::net::TcpListener;

async fn hello_world() -> &'static str {
    "Welcome to SocialTraining!"
}

#[tokio::main]
async fn main() {
    let port = 3001;
    let url = format!("127.0.0.1:{}", port);

    let app = Router::new()
        .route("/", get(hello_world))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );
    let listener = TcpListener::bind(&url).await.unwrap();
    println!("Servidor escuchando en http://{}", url);
    axum::serve(listener, app).await.unwrap();
}
