/*
curl -v http://127.0.0.1:3000
curl -v http://127.0.0.1:3000/hello-json | jq
curl -v http://127.0.0.1:3000/now | jq

todo:
- extract path and body params
- error handling
- middleware
- shared state
- templating (https://docs.rs/minijinja/latest/minijinja/)
*/

use axum::{Router, response::Json, routing::get};
use serde_json::{Value, json};
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn hello_json() -> Json<Value> {
    Json(json!({"message": "Hello, World!", "data": 42}))
}

async fn now() -> Json<Value> {
    Json(json!({"now": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()}))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/hello-json", get(hello_json))
        .route("/now", get(now));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("Serving on http://127.0.0.1:3000 ...");
    axum::serve(listener, app).await?;

    Ok(())
}
