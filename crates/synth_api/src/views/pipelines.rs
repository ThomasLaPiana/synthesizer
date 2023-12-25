use actix_web::{web, HttpResponse};
use askama::Template;
use sqlx::SqlitePool;
use synth_common::models::Pipeline;
use synth_common::queries;

#[derive(Template)]
#[template(path = "pipelines/index.html")]
struct Index {
    pipelines: Vec<Pipeline>,
}

pub async fn index(db_pool: web::Data<SqlitePool>) -> HttpResponse {
    let pipelines = queries::select_pipelines(&db_pool).await.unwrap();
    let index_template = Index { pipelines };
    let rendered_html = index_template.render().unwrap();
    HttpResponse::Ok().body(rendered_html)
}
