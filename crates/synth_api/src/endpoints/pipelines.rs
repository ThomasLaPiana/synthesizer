use crate::models::JSONResponse;
use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;
use synth_common::models::Pipeline;

/// Return a list of all pipelines
pub async fn list(db_pool: web::Data<SqlitePool>) -> HttpResponse {
    let pipelines = sqlx::query_as!(Pipeline, "SELECT * FROM pipelines")
        .fetch_all(db_pool.get_ref())
        .await
        .unwrap();

    let response_data = JSONResponse::<Pipeline> {
        data: Some(pipelines),
        errors: None,
    };
    HttpResponse::Ok().json(response_data)
}

/// Get a specific Pipeline
pub async fn get(path: web::Path<String>, db_pool: web::Data<SqlitePool>) -> HttpResponse {
    let id = path.to_string();
    let pipeline = sqlx::query_as!(Pipeline, "SELECT * FROM pipelines WHERE id = ?", id)
        .fetch_one(db_pool.get_ref())
        .await
        .unwrap();

    let response_data = JSONResponse::<Pipeline> {
        data: Some(vec![pipeline]),
        errors: None,
    };
    HttpResponse::Ok().json(response_data)
}

/// Create a Pipeline
pub async fn create(pipeline: web::Json<Pipeline>, db_pool: web::Data<SqlitePool>) -> HttpResponse {
    sqlx::query!(
        "INSERT INTO pipelines (id, schedule) VALUES(?, ?)",
        pipeline.id,
        pipeline.schedule,
    )
    .execute(db_pool.get_ref())
    .await
    .unwrap();

    HttpResponse::Created().finish()
}
