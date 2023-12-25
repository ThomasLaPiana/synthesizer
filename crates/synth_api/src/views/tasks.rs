use actix_web::{web, HttpResponse};
use askama::Template;
use sqlx::SqlitePool;
use synth_common::models::Task;
use synth_common::queries;

#[derive(Template)]
#[template(path = "tasks/index.html")]
struct Index {
    tasks: Vec<Task>,
}

pub async fn index(db_pool: web::Data<SqlitePool>) -> HttpResponse {
    let tasks = queries::select_tasks(&db_pool).await.unwrap();
    let index_template = Index { tasks };
    let rendered_html = index_template.render().unwrap();
    HttpResponse::Ok().body(rendered_html)
}
