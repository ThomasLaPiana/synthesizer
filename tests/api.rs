use std::net::TcpListener;

use reqwest::{Client, StatusCode};
use synthesizer::{database, models, webserver};

pub async fn setupdb() {
    database::reset_database().await.unwrap();
    database::run_migrations().await.unwrap();
}

/// Spawn an application instance on a random, available
/// port and return the address. The application instance
/// will automatically be destroyed and cleaned when the
/// process ends.
pub fn spawn_app() -> String {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random, available port!");
    let port = listener.local_addr().unwrap().port();
    let _ = tokio::spawn(webserver::run(listener).unwrap());
    format!("http://127.0.0.1:{}", port)
}

mod generic {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn health_success() {
        // Arrange
        let server_address = spawn_app();
        let client = Client::new();
        let url = &format!("{}/api/health", server_address);

        // Act
        let response = client
            .get(url)
            .send()
            .await
            .expect("Failed to send request!");

        // Assert
        assert_eq!(response.status(), StatusCode::OK)
    }

    #[tokio::test]
    async fn get_endpoints() {
        // Arrange
        let server_address = spawn_app();
        let client = Client::new();
        let url = &format!("{}/api/endpoints", server_address);

        // Act
        let response = client
            .get(url)
            .send()
            .await
            .expect("Failed to send request!");

        // Assert
        assert_eq!(response.status(), StatusCode::OK)
    }
}

mod pipelines {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn list_pipelines_success() {
        // Arrange
        let server_address = spawn_app();
        let client = Client::new();
        let url = &format!("{}/api/pipelines", server_address);

        // Act
        let response = client
            .get(url)
            .send()
            .await
            .expect("Failed to send request!");

        // Assert
        assert_eq!(response.status(), StatusCode::OK)
    }

    #[tokio::test]
    async fn create_one_pipeline_success() {
        // Arrange
        setupdb().await;
        let server_address = spawn_app();
        let client = Client::new();
        let url = &format!("{}/api/pipelines", server_address);
        let request_data = models::Pipeline {
            name: Some("testpipeline".to_owned()),
            id: "testpipeline".to_owned(),
            schedule: "1 * * * *".to_owned(),
        };

        // Act
        let response = client
            .post(url)
            .json(&request_data)
            .send()
            .await
            .expect("Failed to send request!");

        // Assert
        assert_eq!(response.status(), StatusCode::CREATED);

        let body: models::Pipeline = response
            .json()
            .await
            .expect("Failed to parse the create pipeline response!");
        assert_eq!(body, request_data);
    }
}

mod tasks {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn create_one_task_success() {
        // Arrange
        setupdb().await;
        let server_address = spawn_app();
        let client = Client::new();
        let url = &format!("{}/api/tasks", server_address);
        let request_data = models::Task {
            name: "testtask".to_owned(),
            pipeline_id: "testpipeline".to_owned(),
            command: "1 * * * *".to_owned(),
        };

        // Act
        let response = client
            .post(url)
            .json(&request_data)
            .send()
            .await
            .expect("Failed to send request!");

        // Assert
        assert_eq!(response.status(), StatusCode::CREATED);

        let body: models::Task = response.json().await.unwrap();
        assert_eq!(body, request_data);
    }

    #[tokio::test]
    async fn list_tasks_success() {
        // Arrange
        let server_address = spawn_app();
        let client = Client::new();
        let url = &format!("{}/api/tasks", server_address);

        // Act
        let response = client
            .get(url)
            .send()
            .await
            .expect("Failed to send request!");

        // Assert
        assert_eq!(response.status(), StatusCode::OK)
    }
}
