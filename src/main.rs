mod entitys;
mod handler;
mod boot_server;
mod service;
mod vo;
mod dao;
pub mod error;

use axum::{
    body::Body, http::Response, response::IntoResponse, routing::{get, post}, Router
};
use handler::user_handler::UserHandler;
use boot_server::state::AppState;
use sea_orm::*;

const DATABASE_URL: &str = "mysql://root:123456@localhost:3306";
const DB_NAME: &str = "rust_demo";

#[tokio::main]
async fn main() {

    let conn = connect_db().await.expect("failed connection db");

    let state = AppState{ conn };

    let app = Router::new()
    .route("/", get(root))
    .route("/users", post(UserHandler::create_user).put(UserHandler::update_user).delete(UserHandler::delete_user))
    .route("/users/:username", get(UserHandler::get_user))
    .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn root() -> Response<Body> {
    "Hello, World!".into_response()
}

async fn connect_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let db = match db.get_database_backend() {
        DatabaseBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(), 
                format!("CREATE DATABASE IF NOT EXISTS `{}`", DB_NAME),
            )).await?;
            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
            Database::connect(&url).await?
        },
        _ => {db}
    };

    Ok(db)
}