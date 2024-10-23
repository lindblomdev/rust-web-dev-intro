use axum::{
    extract::{Path, State},
    routing::{get, patch, post},
    Json, Router,
};

use super::{
    app_state::AppState,
    auth::RequireAuthentication,
    jwt::JWTKeyProvider,
    store::{Store, Todo, ValidUsername},
};

#[derive(serde::Deserialize)]
struct NewTodo {
    pub title: String,
}

pub(crate) fn rest_router() -> Router<AppState> {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/todos", get(get_todos).post(create_todo))
        .route("/todos/:id", patch(update_todo).delete(delete_todo))
}

#[derive(serde::Deserialize)]
struct UserInfo {
    username: String,
    password: String,
}

async fn signup(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    Json(u): Json<UserInfo>,
) -> String {
    if let Ok(id) = store
        .create_user(ValidUsername::from(u.username), u.password)
        .await
    {
        use jwt_simple::prelude::*;

        let mut claims = Claims::create(Duration::from_hours(2));
        claims.subject = Some(id.to_string());
        let token = jwt_key_provider.key_pair.sign(claims);
        token.unwrap_or(String::new())
    } else {
        "User cration failed".into()
    }
}

async fn login(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    Json(u): Json<UserInfo>,
) -> String {
    use jwt_simple::prelude::*;

    let login = store
        .get_authenticated_user_id(ValidUsername::from(u.username), u.password)
        .await
        .unwrap_or(None);

    match login {
        Some(id) => {
            let mut claims = Claims::create(Duration::from_hours(2));
            claims.subject = Some(id.to_string());
            let token = jwt_key_provider.key_pair.sign(claims);
            token.unwrap_or(String::new())
        }
        None => String::new(),
    }
}

async fn get_todos(
    State(store): State<Store>,
    RequireAuthentication(user): RequireAuthentication,
) -> Json<Vec<Todo>> {
    Json(store.get_todos(user.id).await.unwrap()) // <- hardcoded user_id
}

async fn create_todo(
    State(store): State<Store>,
    RequireAuthentication(user): RequireAuthentication,
    Json(todo): Json<NewTodo>,
) -> Json<Vec<Todo>> {
    let todos = store
        .add_todo_and_return_all(user.id, todo.title)
        .await
        .unwrap();
    Json(todos)
}

async fn update_todo(
    State(store): State<Store>,
    Path(id): Path<i32>,
    RequireAuthentication(user): RequireAuthentication,
    Json(todo): Json<Todo>,
) -> Json<Vec<Todo>> {
    let todos = store
        .update_todo_and_return_all(user.id, id, todo.title, todo.completed)
        .await
        .expect("failed to update");
    Json(todos)
}

async fn delete_todo(
    State(store): State<Store>,
    Path(id): Path<i32>,
    RequireAuthentication(user): RequireAuthentication,
) -> Json<Vec<Todo>> {
    let todos = store.delete_todo_and_return_all(user.id, id).await.unwrap();
    Json(todos)
}
