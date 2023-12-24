mod helpers;

use crate::helpers::spawn_app;
use pretty_assertions::assert_eq;
use reqwest::{Client, StatusCode};
use synth_api::models::JSONResponse;
use synth_common::models;

#[tokio::test]
async fn create_one_task_success() {
    // Arrange
    let server_address = spawn_app().await;
    let client = Client::new();

    // Create the objects used in the test
    let id = "testtask".to_owned();

    // Send the POST for creation
    let create_url = &format!("{}/api/tasks", server_address);
    let create_data = models::Task {
        id: id.clone(),
        pipeline_id: "testpipeline".to_owned(),
        command: "1 * * * *".to_owned(),
    };
    let create_response = client
        .post(create_url)
        .json(&create_data)
        .send()
        .await
        .expect("Failed to POST task!");

    // GET the newly created object
    let get_url = &format!("{}/api/tasks/{}", server_address, id);
    let get_data = JSONResponse::<models::Task> {
        data: Some(vec![create_data.clone()]),
        errors: None,
    };
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

    // Assert that the GET was successful
    assert_eq!(
        get_response.status(),
        StatusCode::OK,
        "GET Task request failed!"
    );
    let body: JSONResponse<models::Task> = get_response.json().await.unwrap();
    assert_eq!(body, get_data, "GET Task body unequal!");
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
