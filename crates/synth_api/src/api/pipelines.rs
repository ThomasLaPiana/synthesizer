use crate::models::JSONResponse;
use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;
use synth_common::models::Pipeline;
use synth_common::queries;

/// Return a list of all pipelines
pub async fn list(db_pool: web::Data<SqlitePool>) -> HttpResponse {
    let result = queries::select_pipelines(&db_pool).await;

    match result {
        Ok(_) => {
            let response_data = JSONResponse::<Pipeline> {
                data: None,
                errors: None,
            };
            HttpResponse::Ok().json(response_data)
        }
        Err(_) => {
            let response_data = JSONResponse::<Pipeline> {
                data: None,
                errors: Some(vec!["Failed to get pipelines!".to_string()]),
            };
            HttpResponse::InternalServerError().json(response_data)
        }
    }
}

/// Get a specific Pipeline
pub async fn get(path: web::Path<String>, db_pool: web::Data<SqlitePool>) -> HttpResponse {
    let id = path.to_string();
    let result = queries::select_pipeline_by_id(&id, &db_pool).await;

    match result {
        Ok(pipeline) => {
            let response_data = JSONResponse::<Pipeline> {
                data: Some(vec![pipeline]),
                errors: None,
            };
            HttpResponse::Ok().json(response_data)
        }
        Err(_) => {
            let response_data = JSONResponse::<Pipeline> {
                data: None,
                errors: Some(vec!["Failed to get pipeline!".to_string()]),
            };
            HttpResponse::InternalServerError().json(response_data)
        }
    }
}

/// Create a Pipeline
pub async fn create(pipeline: web::Json<Pipeline>, db_pool: web::Data<SqlitePool>) -> HttpResponse {
    let pipeline = pipeline.into_inner();
    let result = queries::upsert_pipeline(&pipeline, &db_pool).await;

    match result {
        Ok(_) => {
            let response_data = JSONResponse::<Pipeline> {
                data: Some(vec![pipeline]),
                errors: None,
            };
            HttpResponse::Created().json(response_data)
        }
        Err(_) => {
            let response_data = JSONResponse::<Pipeline> {
                data: None,
                errors: Some(vec!["Failed to create the pipeline!".to_string()]),
            };
            HttpResponse::InternalServerError().json(response_data)
        }
    }
}
