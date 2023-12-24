use chrono::{DateTime, Utc};
use cron_parser::parse;
use sqlx::{self, Pool, Sqlite};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use synth_common::database;
use synth_common::models::{Pipeline, Task};
use tracing::{error, info, instrument, span, Level};

async fn async_sleep(sleep_secs: u64) {
    let sleep_duration = tokio::time::Duration::from_secs(sleep_secs);
    tokio::time::sleep(sleep_duration).await;
}

#[instrument(name = "PipelineRunner", skip_all)]
async fn pipeline_runner(pipeline: Pipeline, scheduled_time: DateTime<Utc>, db_pool: Pool<Sqlite>) {
    info!("Running Pipeline: {}", &pipeline.id);
    let pipeline_instance = format!("{}_{}", pipeline.id, scheduled_time);
    let pipeline_id = pipeline.id.clone();

    let tasks: Vec<Task> = sqlx::query_as!(
        Task,
        "SELECT * FROM tasks WHERE pipeline_id = ?",
        pipeline.id
    )
    .fetch_all(&db_pool)
    .await
    .unwrap();

    info!("Executing Pipeline Instance: {}", pipeline_instance);
    // Spawn a new thread to handle the Pipeline's tasks
    tokio::task::spawn(async move {
        for task in tasks {
            let span = span!(Level::INFO, "TaskRunner");
            let _enter = span.enter();
            info!(
                "Task '{}' for Pipeline '{}' has started!",
                task.id, pipeline_id
            );
            // Run the Task subprocess
            let task_process = Command::new("sh")
                .arg("-c")
                .arg(task.command)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Task failed to start!");
            let output = task_process
                .wait_with_output()
                .expect("Failed to wait on the Task!");

            info!("Saving to database...");
            let output_status = output.status.to_string();
            let output_logs = output.stdout;
            // TODO: add Stderr
            let created_at = &Utc::now();
            let task_instance = format!("{}_{}", task.id, pipeline_instance);
            sqlx::query!(
                "INSERT INTO task_instances (id, task_id, pipeline_id, execution_time, status, logs, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
                task_instance,
                task.id,
                pipeline_id,
                scheduled_time,
                output_status,
                output_logs,
                created_at,
            )
            .execute(&db_pool)
            .await
            .unwrap();

            // Store the results in the database
            if output.status.success() {
                info!("Task succeeded!");
            } else {
                error!("Task failed! Stopping Pipeline.");
                break;
            }
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
