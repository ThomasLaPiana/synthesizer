use super::models::{Pipeline, Task, TaskInstance};
use sqlx::{self, Pool, Sqlite};

/// Insert a TaskInstance into the database
pub async fn insert_task_instance(
    task_instance: TaskInstance,
    db_pool: &Pool<Sqlite>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
                "INSERT INTO task_instances (id, task_id, pipeline_id, scheduled_time, execution_start, execution_end, status, logs, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
                task_instance.id,
                task_instance.task_id,
                task_instance.pipeline_id,
                task_instance.scheduled_time,
                task_instance.execution_start,
                task_instance.execution_end,
                task_instance.status,
                task_instance.logs,
                task_instance.created_at,
            )
            .execute(db_pool)
            .await?;
    Ok(())
}

/// Upsert a Pipeline
pub async fn upsert_pipeline(
    pipeline: &Pipeline,
    db_pool: &Pool<Sqlite>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO pipelines (id, schedule) VALUES(?, ?) ON CONFLICT(id) DO UPDATE SET schedule = ?",
        pipeline.id,
        pipeline.schedule,
        pipeline.schedule,
    )
    .execute(db_pool)
    .await?;
    Ok(())
}

/// Insert a Pipeline
pub async fn insert_pipeline(
    pipeline: Pipeline,
    db_pool: &Pool<Sqlite>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO pipelines (id, schedule) VALUES(?, ?)",
        pipeline.id,
        pipeline.schedule,
    )
    .execute(db_pool)
    .await?;
    Ok(())
}

/// Insert a Task
pub async fn insert_task(task: Task, db_pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO tasks (id, pipeline_id, command) VALUES(?, ?, ?)",
        task.id,
        task.pipeline_id,
        task.command,
    )
    .execute(db_pool)
    .await?;
    Ok(())
}

/// Upsert a Task
pub async fn upsert_task(task: &Task, db_pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO tasks (id, pipeline_id, command) VALUES(?, ?, ?) ON CONFLICT(id) DO UPDATE SET pipeline_id = ?, command = ?",
        task.id,
        task.pipeline_id,
        task.command,
        task.pipeline_id,
        task.command,
    )
    .execute(db_pool)
    .await?;
    Ok(())
}

/// Select Task by Pipeline ID
pub async fn select_task_by_pipeline_id(
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

/// Get all Tasks
pub async fn select_tasks(db_pool: &Pool<Sqlite>) -> Result<Vec<Task>, sqlx::Error> {
    let tasks = sqlx::query_as!(Task, "SELECT * FROM tasks ORDER BY pipeline_id")
        .fetch_all(db_pool)
        .await?;
    Ok(tasks)
}

/// Get all Pipelines
pub async fn select_pipelines(db_pool: &Pool<Sqlite>) -> Result<Vec<Pipeline>, sqlx::Error> {
    let pipelines = sqlx::query_as!(Pipeline, "SELECT * FROM pipelines",)
        .fetch_all(db_pool)
        .await?;
    Ok(pipelines)
}

/// Get a Pipeline by ID
pub async fn select_pipeline_by_id(
    pipeline_id: &str,
    db_pool: &Pool<Sqlite>,
) -> Result<Pipeline, sqlx::Error> {
    let pipeline = sqlx::query_as!(
        Pipeline,
        "SELECT * FROM pipelines WHERE id = ?",
        pipeline_id
    )
    .fetch_one(db_pool)
    .await?;
    Ok(pipeline)
}
