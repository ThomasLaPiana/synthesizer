use crate::helpers::spawn_app;
use pretty_assertions::assert_eq;
use reqwest::{Client, StatusCode};
use synthesizer::api::models::JSONResponse;
use synthesizer::common::models;

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
    let id = "testpipeline".to_string();

    // Send the POST for creation
    let create_url = &format!("{}/api/pipelines", server_address);
    let create_data = models::Pipeline {
        id: id.clone(),
        schedule: "1 * * * *".to_owned(),
    };
    let create_response = client
        .post(create_url)
        .json(&create_data)
        .send()
        .await
        .expect("Failed to POST pipeline!");

    // GET the newly created object
    let get_url = &format!("{}/api/pipelines/{}", server_address, id);
    let get_data = JSONResponse::<models::Pipeline> {
        data: Some(vec![create_data.clone()]),
        errors: None,
    };
    let get_response = client
        .get(get_url)
        .send()
        .await
        .expect("Failed to GET pipeline!");

    // Assert that the POST was successful
    assert_eq!(
        create_response.status(),
        StatusCode::CREATED,
        "POST Pipeline request failed!"
    );

    // Assert that the GET was successful
    assert_eq!(
        get_response.status(),
        StatusCode::OK,
        "GET Pipeline request failed!"
    );
    let body: JSONResponse<models::Pipeline> = get_response.json().await.unwrap();
    assert_eq!(body, get_data, "GET Pipeline body unequal!");
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
