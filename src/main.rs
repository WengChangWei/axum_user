mod entitys;
mod handler;
mod boot_server;

use axum::{
    routing::{get, post}, 
    Router
};
use handler::user_handler::UserHandler;
use boot_server::state::{self, AppState};
use sea_orm::*;

const DATABASE_URL: &str = "mysql://root:123456@localhost:3306";
const DB_NAME: &str = "rust_demo";

#[tokio::main]
async fn main() {

    let conn = connect_db().await.expect("failed connection db");

    let state = AppState{ conn };

    let app = Router::new()
    .route("/", get(UserHandler::root))
    .route("/users", post(UserHandler::create_user).put(UserHandler::update_user).delete(UserHandler::delete_user))
    .route("/users/:username", get(UserHandler::get_user))
    .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
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