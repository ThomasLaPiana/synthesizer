use super::models::JSONResponse;
use crate::common::models::Task;
use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;

/// Return a list of all Tasks
pub async fn list_tasks(db_pool: web::Data<SqlitePool>) -> HttpResponse {
    let tasks = sqlx::query_as!(Task, "SELECT * FROM tasks")
        .fetch_all(db_pool.get_ref())
        .await
        .unwrap();

    let response_data = JSONResponse::<Task> {
        data: Some(tasks),
        errors: None,
    };
    HttpResponse::Ok().json(response_data)
}

/// Get a specific Task
pub async fn get_task(path: web::Path<String>, db_pool: web::Data<SqlitePool>) -> HttpResponse {
    let id = path.to_string();
    let task = sqlx::query_as!(Task, "SELECT * FROM tasks WHERE id = ?", id)
        .fetch_one(db_pool.get_ref())
        .await
        .unwrap();

    let response_data = JSONResponse::<Task> {
        data: Some(vec![task]),
        errors: None,
    };
    HttpResponse::Ok().json(response_data)
}

/// Create a Task
pub async fn create_task(task: web::Json<Task>, db_pool: web::Data<SqlitePool>) -> HttpResponse {
    sqlx::query!(
        "INSERT INTO tasks (id, pipeline_id, command) VALUES(?, ?, ?)",
        task.id,
        task.pipeline_id,
        task.command,
    )
    .execute(db_pool.get_ref())
    .await
    .unwrap();

    HttpResponse::Created().finish()
}
