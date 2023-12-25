use super::models::{Task, TaskInstance};
use sqlx::{self, Pool, Sqlite};

/// Insert a TaskInstance into the database
pub async fn insert_task_instance(
    task_instance: TaskInstance,
    db_pool: &Pool<Sqlite>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
                "INSERT INTO task_instances (id, task_id, pipeline_id, execution_time, status, logs, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
                task_instance.id,
                task_instance.task_id,
                task_instance.pipeline_id,
                task_instance.execution_time,
                task_instance.status,
                task_instance.logs,
                task_instance.created_at,
            )
            .execute(db_pool)
            .await?;
    Ok(())
}

/// Select TaskInstances by Pipeline ID
pub async fn select_task_instances_by_pipeline_id(
    pipeline_id: &str,
    db_pool: &Pool<Sqlite>,
) -> Result<Vec<Task>, sqlx::Error> {
    let tasks: Vec<Task> = sqlx::query_as!(
        Task,
        "SELECT * FROM tasks WHERE pipeline_id = ?",
        pipeline_id
    )
    .fetch_all(db_pool)
    .await?;
    Ok(tasks)
}
