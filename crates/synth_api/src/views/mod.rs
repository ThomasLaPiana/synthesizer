pub mod pipelines;
pub mod task_instances;

use actix_web::HttpResponse;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

pub async fn index() -> HttpResponse {
    let rendered_html = Index.render().unwrap();
    HttpResponse::Ok().body(rendered_html)
}
