use axum::{Router, extract::Path, routing::get};

async fn hello_world() -> &'static str {
    "Hello, World!"
}

/// Say hello to a user
async fn hello_user(Path(user): Path<String>) -> String {
    format!("Hello, {}!", user)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/{user}", get(hello_user));

    let addr = "0.0.0.0:3000".to_string();
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
