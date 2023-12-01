use crate::common::models::Pipeline;
use sqlx;
use sqlx::SqlitePool;
use tracing::{info, instrument};

#[instrument(name = "Scheduler", skip_all)]
pub async fn run_scheduler(db_pool: SqlitePool) {
    loop {
        info!("Loading pipelines from db...");
        let pipelines: Vec<Pipeline> = sqlx::query_as!(Pipeline, "SELECT * FROM pipelines")
            .fetch_all(&db_pool)
            .await
            .unwrap();
        info!("Loaded '{:?}' pipelines!", pipelines.len());

        for pipeline in pipelines {
            info!("Found the cron time: {:?}", pipeline.schedule)
        }
        // Sleep a tad
        let sleep_duration = tokio::time::Duration::from_secs(5);
        tokio::time::sleep(sleep_duration).await;
    }

    // Loop through them to figure out when the next runtime is

    // If it is ready to run, pass it to an execution thread

    // check the results of the execution thread, store the logs/status in the database
}
