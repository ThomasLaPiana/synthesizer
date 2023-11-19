use synthesizer::database;

pub async fn setupdb() {
    database::reset_database().await.unwrap();
    database::run_migrations().await.unwrap();
}
