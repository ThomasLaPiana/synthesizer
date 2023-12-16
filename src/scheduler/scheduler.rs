use crate::common::models::{Pipeline, Task};
use crate::common::{database, telemetry};
use sqlx;
use std::collections::HashMap;
use std::process::Command;
use tracing::{info, instrument};

async fn async_sleep(sleep_secs: u64) {
    let sleep_duration = tokio::time::Duration::from_secs(sleep_secs);
    tokio::time::sleep(sleep_duration).await;
}

#[instrument(name = "Scheduler", skip_all)]
pub async fn run_scheduler() {
    telemetry::init_logging();
    let db_pool = database::get_db_pool().await;

    // In-memory map of the tasks and their next execution time
    let mut pipeline_schedules: HashMap<String, String> = HashMap::new();

    // This never-ending loop is the scheduler
    loop {
        info!("Loading pipelines from db...");
        let pipelines: Vec<Pipeline> = sqlx::query_as!(Pipeline, "SELECT * FROM pipelines")
            .fetch_all(&db_pool)
            .await
            .unwrap();

        for pipeline in pipelines {
            let pipeline_id = pipeline.id.clone();
            if pipeline_schedules.get(&pipeline.id).is_none() {
                pipeline_schedules.insert(pipeline.id, pipeline.schedule);
            }

            // If it is time for execution, load the tasks
            let tasks = sqlx::query_as!(
                Task,
                "SELECT * FROM tasks WHERE pipeline_id = ?",
                pipeline_id
            )
            .fetch_all(&db_pool)
            .await
            .unwrap();

            // Spawn a new thread to handle each Pipeline's tasks
            tokio::task::spawn(async {
                for task in tasks {
                    let mut result = Command::new("sh")
                        .arg("-c")
                        .arg(task.command)
                        .spawn()
                        .expect("Command failed!");
                    result.wait().expect("Task failed!");
                }
            });
        }

        info!("{:?}", pipeline_schedules);
        // Sleep a tad to avoid resource saturation
    }
}
