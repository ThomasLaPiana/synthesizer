use crate::models::JSONResponse;
use actix_web::HttpResponse;

/// Standard health endpoint
pub async fn health() -> HttpResponse {
    let response_data = JSONResponse::<String> {
        data: Some(vec!["Feeling healthy!".to_string()]),
        errors: None,
    };
    HttpResponse::Ok().json(response_data)
}
