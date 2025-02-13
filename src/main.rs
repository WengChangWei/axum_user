use axum::{
    body::Body, 
    extract::{rejection::FormRejection, Form, FromRequest, Path, Request}, 
    http::StatusCode, 
    response::{IntoResponse, Response}, 
    routing::{get, post}, 
    Json, 
    Router
};
use serde::{de::{DeserializeOwned}, Deserialize, Serialize};
use validator::Validate;
use thiserror::Error;

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


#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedForm(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),

}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(_) => {
                let message = format!("Input valication error: [{self}]").replace('\n', ",");
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        }
        .into_response()
    }
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