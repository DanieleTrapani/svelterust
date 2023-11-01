use axum::{
    extract::{Path, State},
    routing::get,
    Form, Json, Router,
};
use axum_error::Result;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<()> {
    // ENV Variables
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&url).await?;

    static PORT: u16 = 3000;
    let app = Router::new()
        .route("/", get(list))
        .route("/create", get(create))
        .route("/delete/:id", get(delete))
        .route("/update", get(update))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());
    let address = SocketAddr::from(([0, 0, 0, 0], PORT));
    println!("Listening on http://localhost:{PORT}/");

    Ok(axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?)
}

#[derive(Deserialize, Serialize)]
struct Todo {
    id: i64,
    description: String,
    done: bool,
}

#[derive(Deserialize)]
struct NewTodo {
    description: String,
}

async fn list(State(pool): State<SqlitePool>) -> Result<Json<Vec<Todo>>> {
    let todos = sqlx::query_as!(Todo, "SELECT * FROM todos ORDER BY id")
        .fetch_all(&pool)
        .await?;
    Ok(Json(todos))
}

async fn create(State(pool): State<SqlitePool>, Form(todo): Form<NewTodo>) -> Result<String> {
    sqlx::query!(
        "INSERT INTO todos (description) VALUES (?)",
        todo.description
    )
    .execute(&pool)
    .await?;
    Ok(format!("Created todo!"))
}

async fn update(State(pool): State<SqlitePool>, Form(todo): Form<Todo>) -> Result<String> {
    sqlx::query!(
        "UPDATE todos SET description = ?, done = ? WHERE id = ?",
        todo.description,
        todo.done,
        todo.id
    )
    .execute(&pool)
    .await?;
    Ok(format!("Updated todo!"))
}

async fn delete(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<String> {
    sqlx::query!("DELETE FROM todos WHERE id = ?", id)
        .execute(&pool)
        .await?;
    Ok(format!("Deleted todo!"))
}
