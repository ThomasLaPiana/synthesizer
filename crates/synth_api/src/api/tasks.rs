use crate::models::JSONResponse;
use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;
use synth_common::models::Task;
use synth_common::queries;

/// Return a list of all Tasks
pub async fn list(db_pool: web::Data<SqlitePool>) -> HttpResponse {
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
pub async fn get(path: web::Path<String>, db_pool: web::Data<SqlitePool>) -> HttpResponse {
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
pub async fn create(task: web::Json<Task>, db_pool: web::Data<SqlitePool>) -> HttpResponse {
    let result = queries::upsert_task(&task.into_inner(), &db_pool).await;

    match result {
        Ok(_) => {
            let response_data = JSONResponse::<Task> {
                data: None,
                errors: None,
            };
            HttpResponse::Created().json(response_data)
        }
        Err(_) => {
            let response_data = JSONResponse::<Task> {
                data: None,
                errors: Some(vec!["Failed to create the task!".to_string()]),
            };
            HttpResponse::InternalServerError().json(response_data)
        }
    }
}
