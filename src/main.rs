use axum::{
    body::Body, extract::Path, http::StatusCode, response::{IntoResponse, Response}, routing::{get, post}, Json, Router
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(root))
    .route("/users", post(create_user))
    .route("/users/:username", get(get_user));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Response<Body> {
    "Hello, World!".into_response()
}

async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

async fn get_user(Path(name): Path<String>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: name,
    };

    (StatusCode::OK, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}