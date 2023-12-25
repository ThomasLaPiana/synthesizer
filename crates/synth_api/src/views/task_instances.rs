use actix_web::{web, HttpResponse};
use askama::Template;
use sqlx::SqlitePool;
use synth_common::models::TaskInstance;

#[derive(Template)]
#[template(path = "task_instances/index.html")]
struct Index {
    task_instances: Vec<TaskInstance>,
}

pub async fn index(db_pool: web::Data<SqlitePool>) -> HttpResponse {
    let task_instances = sqlx::query_as!(TaskInstance, "SELECT * FROM task_instances")
        .fetch_all(db_pool.get_ref())
        .await
        .unwrap();
    let index_template = Index { task_instances };

    let rendered_html = index_template.render().unwrap();
    HttpResponse::Ok().body(rendered_html)
}
