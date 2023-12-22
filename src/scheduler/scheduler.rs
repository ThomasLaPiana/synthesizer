use crate::common::database;
use crate::common::models::{Pipeline, Task};
use chrono::{DateTime, Utc};
use cron_parser::parse;
use sqlx::{self, Pool, Sqlite};
use std::collections::HashMap;
use std::process::Command;
use tracing::{info, instrument};

async fn async_sleep(sleep_secs: u64) {
    let sleep_duration = tokio::time::Duration::from_secs(sleep_secs);
    tokio::time::sleep(sleep_duration).await;
}

#[instrument(name = "PipelineRunner", skip_all)]
async fn pipeline_runner(
    pipeline: &Pipeline,
    current_run_time: &DateTime<Utc>,
    db_pool: &Pool<Sqlite>,
) {
    info!("Running Pipeline: {}", &pipeline.id);

    let tasks: Vec<Task> = sqlx::query_as!(
        Task,
        "SELECT * FROM tasks WHERE pipeline_id = ?",
        pipeline.id
    )
    .fetch_all(db_pool)
    .await
    .unwrap();

    info!(
        "Running Pipeline Instance: {}_{}",
        pipeline.id, current_run_time
    );

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

#[instrument(name = "Scheduler", skip_all)]
pub async fn run_scheduler() {
    let db_pool = database::get_db_pool().await;

    // In-memory map of the pipelines and their next execution time
    let mut pipeline_schedules: HashMap<String, DateTime<Utc>> = HashMap::new();

    // This never-ending loop is the scheduler
    loop {
        let now = &Utc::now();
        info!("------------------------------");
        info!("Current time: {}", now);
        info!("Loading Pipelines from db...");
        let pipelines: Vec<Pipeline> = sqlx::query_as!(Pipeline, "SELECT * FROM pipelines")
            .fetch_all(&db_pool)
            .await
            .unwrap();

        for pipeline in pipelines {
            let next: DateTime<Utc> = parse(&pipeline.schedule, &Utc::now()).unwrap();
            let current_scheduled_time = pipeline_schedules.insert(pipeline.id.clone(), next);

            // Handle new Pipelines
            if current_scheduled_time.is_none() {
                info!("Added '{}' to the HashMap...", pipeline.id);
                continue;
            }

            let current_scheduled_time = current_scheduled_time.unwrap();
            let requires_execution = current_scheduled_time < *now;

            if requires_execution {
                info!("Pipeline '{}' is ready for execution!", pipeline.id);
                pipeline_schedules.insert(pipeline.id.clone(), next);
                pipeline_runner(&pipeline, &current_scheduled_time, &db_pool).await;
            } else {
                info!(
                    "Pipeline '{}' is not ready for execution! Next execution at: {}",
                    pipeline.id, current_scheduled_time
                );
            }
        }

        info!("{:?}", pipeline_schedules);
        // Sleep a tad to avoid resource saturation
        async_sleep(5).await;
    }
}
