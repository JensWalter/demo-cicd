use axum::{extract::Path, routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn hello_user(Path(user): Path<String>) -> String {
    format!("Hello, {}!", user)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/{user}", get(hello_user));

    let addr = format!("0.0.0.0:3000");
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
    }
