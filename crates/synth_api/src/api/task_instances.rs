use crate::models::JSONResponse;
use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;
use synth_common::models::TaskInstance;

pub async fn list(db_pool: web::Data<SqlitePool>) -> HttpResponse {
    let task_instances = sqlx::query_as!(TaskInstance, "SELECT * FROM task_instances")
        .fetch_all(db_pool.get_ref())
        .await
        .unwrap();

    let response_data = JSONResponse::<TaskInstance> {
        data: Some(task_instances),
        errors: None,
    };
    HttpResponse::Ok().json(response_data)
}

pub async fn get(path: web::Path<String>, db_pool: web::Data<SqlitePool>) -> HttpResponse {
    let id = path.to_string();
    let task_instance = sqlx::query_as!(
        TaskInstance,
        "SELECT * FROM task_instances WHERE id = ?",
        id
    )
    .fetch_one(db_pool.get_ref())
    .await
    .unwrap();

    let response_data = JSONResponse::<TaskInstance> {
        data: Some(vec![task_instance]),
        errors: None,
    };
    HttpResponse::Ok().json(response_data)
}
