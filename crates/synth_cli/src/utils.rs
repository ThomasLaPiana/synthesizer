use reqwest::Client;

/// Load a file into a String
pub fn load_file(file_path: &str) -> String {
    match std::fs::read_to_string(file_path) {
        Ok(result) => result,
        Err(result) => {
            println!(
                "> Failed to load file '{}' with error: {:#?}",
                file_path,
                result.to_string()
            );
            std::process::exit(2)
        }
    }
}

/// Check that a provided URL is reachable and the status code denotes success
pub fn check_url_reachable_and_success(url: &str) -> bool {
    // Verify that the server was reachable
    let result = reqwest::blocking::get(url);
    if result.is_err() {
        println!("Request failed to url: {}", url);
        return false;
    }

    // Verify that the status code indicates success
    let status = result.unwrap().status();
    if status.is_success() {
        return true;
    }
    false
}

/// POST a JSON object to a URL
pub async fn post_json(
    url: &str,
    json_data: &serde_json::Value,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::new();
    let result = client.post(url).json(json_data).send().await?;
    match result.error_for_status_ref() {
        Ok(_) => Ok(result),
        Err(e) => Err(e),
    }
}
