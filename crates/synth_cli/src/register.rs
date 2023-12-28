use super::models::Manifest;
use super::utils;
use serde_json::{json, Value};
use synth_common::models::{Pipeline, Task};

/// Send the objects within the manifest to the webserver.
pub async fn register(url: &str, manifest: Manifest) -> bool {
    let pipeline_url = format!("{}/api/pipelines", url);
    let task_url = format!("{}/api/tasks", url);
    for manifest_pipeline in manifest.pipelines {
        let pipeline = json!(Pipeline {
            id: manifest_pipeline.id.clone(),
            schedule: manifest_pipeline.schedule,
        });
        let result = utils::post_json(&pipeline_url, &pipeline).await;
        match result {
            Ok(_) => println!("> Successfully POSTed to '{}'", &pipeline_url),
            Err(e) => {
                println!("> Failed POST to '{}'!", &pipeline_url);
                println!("{:?}", e);
            }
        }

        let tasks: Vec<Value> = manifest_pipeline
            .tasks
            .into_iter()
            .map(|task| {
                json!(Task {
                    id: task.id,
                    pipeline_id: manifest_pipeline.id.clone(),
                    command: task.command,
                })
            })
            .collect();

        for task in tasks {
            let result = utils::post_json(&task_url, &task).await;
            match result {
                Ok(_) => println!("> Successfully POSTed to '{}'", &task_url),
                Err(e) => {
                    println!("> Failed POST to '{}'!", &task_url);
                    println!("{:?}", e);
                }
            }
        }
    }

    true
}
