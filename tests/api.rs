use std::net::TcpListener;

use reqwest::{Client, StatusCode};
use sqlx::SqlitePool;
use synthesizer::{
    config::{self, BuildUrl},
    database, models, webserver,
};
use uuid::Uuid;

/// Spawn an application instance on a random, available
/// port and return the address. The application instance
/// will automatically be destroyed and cleaned when the
/// process ends.
pub async fn spawn_app() -> String {
    // Init values for configuration
    let mut config = config::load_config("synth.toml").expect("Failed to load configuration!");
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random, available port!");
    let port = listener.local_addr().unwrap().port();
    config.database.database = format!("test-{}", Uuid::new_v4().to_string());
    let db_url = &config.database.build_url();

    // Prepare the database and pool
    database::setupdb(db_url)
        .await
        .expect("Failed to setup test database!");
    let db_pool = SqlitePool::connect(db_url)
        .await
        .expect("Failed to create the database pool!");

    // Run the application instance
    let _ = tokio::spawn(webserver::run(listener, db_pool).unwrap());
    format!("http://127.0.0.1:{}", port)
}

mod generic {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn health_success() {
        // Arrange
        let server_address = spawn_app().await;
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
        let server_address = spawn_app().await;
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
        let server_address = spawn_app().await;
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
        let server_address = spawn_app().await;
        let client = Client::new();
        let url = &format!("{}/api/pipelines", server_address);
        let request_data = models::Pipeline {
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

    #[tokio::test]
    async fn create_pipeline_failures() {
        // Arrange
        let server_address = spawn_app().await;
        let client = Client::new();
        let url = &format!("{}/api/pipelines", server_address);
        let request_data = vec![
            r#"{"id": "testpipline", "scheduled": "1 * * * *"}"#,
            r#"{"name": "whew", "id": "whewpipeline}"#,
        ];

        for pipeline in request_data {
            // Act
            let response = client
                .post(url)
                .json(&pipeline)
                .send()
                .await
                .expect("Failed to send request!");

            // Assert
            assert_eq!(
                response.status(),
                StatusCode::BAD_REQUEST,
                "Didn't get a 400 with payload: {}",
                pipeline
            );
        }
    }
}

mod tasks {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn create_one_task_success() {
        // Arrange
        let server_address = spawn_app().await;
        let client = Client::new();

        // Create the objects used in the test
        let id = "testtask".to_owned();
        let request_data = models::Task {
            id: id.clone(),
            pipeline_id: "testpipeline".to_owned(),
            command: "1 * * * *".to_owned(),
        };
        let response_data = webserver::JSONResponse::<models::Task> {
            data: Some(request_data.clone()),
            errors: None,
        };

        // Send the POST for creation
        let create_url = &format!("{}/api/tasks", server_address);
        let create_response = client
            .post(create_url)
            .json(&request_data)
            .send()
            .await
            .expect("Failed to POST task!");

        // GET the newly created object
        let get_url = &format!("{}/api/tasks/{}", server_address, id);
        let get_response = client
            .get(get_url)
            .send()
            .await
            .expect("Failed to GET task!");

        // Assert that the POST was successful
        assert_eq!(
            create_response.status(),
            StatusCode::CREATED,
            "POST Task request failed!"
        );
        let body: models::Task = create_response.json().await.unwrap();
        assert_eq!(body, request_data, "POST Task body unequal!");

        // Assert that the GET was successful
        assert_eq!(
            get_response.status(),
            StatusCode::OK,
            "GET Task request failed!"
        );
        let body: webserver::JSONResponse<models::Task> = get_response.json().await.unwrap();
        assert_eq!(body, response_data, "GET Task body unequal!");
    }

    #[tokio::test]
    async fn create_task_failures() {
        // Arrange
        let server_address = spawn_app().await;
        let client = Client::new();
        let url = &format!("{}/api/tasks", server_address);
        let request_data = vec![
            r#"{"name": "testpipline", "command": "1 * * * *", "pipeline_id": 4}"#,
            r#"{"name": "whew", "pipeline_id": "whewpipeline}"#,
        ];

        for data in request_data {
            // Act
            let response = client
                .post(url)
                .json(&data)
                .send()
                .await
                .expect("Failed to send request!");

            // Assert
            assert_eq!(
                response.status(),
                StatusCode::BAD_REQUEST,
                "Didn't get a 400 with payload: {}",
                data
            );
        }
    }

    #[tokio::test]
    async fn list_tasks_success() {
        // Arrange
        let server_address = spawn_app().await;
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
