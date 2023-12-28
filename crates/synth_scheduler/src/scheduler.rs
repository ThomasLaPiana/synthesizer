use chrono::{DateTime, Utc};
use cron_parser::parse;
use sqlx::{self, Pool, Sqlite};
use std::collections::HashMap;
use std::process::{Command, Output, Stdio};
use synth_common::models::{Pipeline, TaskInstance};
use synth_common::{database, queries};
use tracing::{error, info, span, Level};

async fn async_sleep(sleep_secs: u64) {
    let sleep_duration = tokio::time::Duration::from_secs(sleep_secs);
    tokio::time::sleep(sleep_duration).await;
}

fn run_task_command(task_command: &str) -> Output {
    Command::new("sh")
        .arg("-c")
        .arg(task_command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Task failed to start!")
        .wait_with_output()
        .expect("Failed to wait on the Task!")
}

async fn pipeline_runner(pipeline: Pipeline, scheduled_time: DateTime<Utc>, db_pool: Pool<Sqlite>) {
    let span = span!(Level::INFO, "PipelineRunner");
    let _enter = span.enter();
    info!("Running Pipeline: {}", &pipeline.id);
    let pipeline_instance = format!("{}_{}", pipeline.id, scheduled_time);
    let pipeline_id = pipeline.id.clone();

    let tasks = queries::select_task_by_pipeline_id(&pipeline_id, &db_pool)
        .await
        .unwrap();

    info!("Executing Pipeline Instance: {}", pipeline_instance);
    // Spawn a new thread to handle the Pipeline's tasks
    tokio::task::spawn(async move {
        let span = span!(Level::INFO, "TaskRunner");
        let _enter = span.enter();
        for task in tasks {
            let execution_start = Utc::now().to_string();
            info!(
                "Task '{}' for Pipeline '{}' has started!",
                task.id, pipeline_id
            );
            // Run the Task subprocess
            let result = run_task_command(&task.command);
            let execution_end = Utc::now().to_string();

            let task_instance_id = format!("{}_{}_{}", task.id, pipeline_id, scheduled_time);
            let task_instance = TaskInstance {
                id: task_instance_id,
                task_id: task.id,
                execution_start,
                execution_end,
                pipeline_id: pipeline_id.clone(),
                scheduled_time: scheduled_time.to_string(),
                status: result.status.to_string(),
                // TODO: add Stderr
                logs: format!("{:?}", result.stdout),
                created_at: Utc::now().to_string(),
            };
            info!("Saving to database...");
            queries::insert_task_instance(task_instance, &db_pool)
                .await
                .unwrap();

            // Store the results in the database
            if result.status.success() {
                info!("Task succeeded!");
            } else {
                error!("Task failed! Stopping Pipeline.");
                break;
            }
        }
    });
}

pub async fn run_scheduler() {
    let span = span!(Level::INFO, "Scheduler");
    let _enter = span.enter();
    let db_pool = database::get_db_pool().await;

    // In-memory map of the pipelines and their next execution time
    // TODO: Move this to a database table?
    let mut pipeline_schedules: HashMap<String, DateTime<Utc>> = HashMap::new();

    // This infinite loop is the scheduler
    loop {
        info!("------------------------------");
        let pipelines: Vec<Pipeline> = sqlx::query_as!(Pipeline, "SELECT * FROM pipelines")
            .fetch_all(&db_pool)
            .await
            .unwrap();

        // NOTE: Easily parallelizable
        for pipeline in pipelines {
            // The 'parse' method returns the next execution time
            let next_scheduled_time: DateTime<Utc> =
                parse(&pipeline.schedule, &Utc::now()).unwrap();
            let current_scheduled_time =
                pipeline_schedules.insert(pipeline.id.clone(), next_scheduled_time);

            // Handle new Pipelines
            if current_scheduled_time.is_none() {
                info!(
                    "Added '{}' to the HashMap! Next execution at: {}",
                    pipeline.id, next_scheduled_time
                );
                continue;
            }

            let current_scheduled_time = current_scheduled_time.unwrap();
            let requires_execution = current_scheduled_time != next_scheduled_time;

            if requires_execution {
                info!("Pipeline '{}' is ready for execution!", pipeline.id);
                pipeline_schedules.insert(pipeline.id.clone(), next_scheduled_time);
                pipeline_runner(pipeline.clone(), current_scheduled_time, db_pool.clone()).await;
            }
        }

        // Sleep a tad to avoid resource saturation
        async_sleep(5).await;
    }
}
