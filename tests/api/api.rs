use crate::helpers::spawn_app;
use pretty_assertions::assert_eq;
use reqwest::{Client, StatusCode};
use synthesizer::server::models::JSONResponse;

#[tokio::test]
async fn health_success() {
    // Arrange
    let server_address = spawn_app().await;
    let client = Client::new();
    let url = &format!("{}/api/health", server_address);

    // Verify the Status
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to send request!");
    assert_eq!(response.status(), StatusCode::OK);

    // Verify the Response Data
    let body: JSONResponse<String> = response
        .json()
        .await
        .expect("Failed to parse the health response!");
    assert_eq!(body.data, Some(vec!["Feeling healthy!".to_string()]));
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
