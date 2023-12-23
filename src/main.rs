use reqwest::{Client, header::{HeaderMap, HeaderValue, USER_AGENT}};
use serde::Serialize;

#[derive(Serialize)]
struct UpdateName {
    name: String,
}

#[tokio::main]
async fn main() {
    let new_name = "nickname"; 

    let access_token = "(((token)))";
    let api_url = "https://api.github.com/user";

    let update_data = UpdateName {
        name: new_name.to_string(),
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", access_token)).expect("Token invalid"),
    );

    headers.insert(USER_AGENT, HeaderValue::from_static("change username")); 

    let client = Client::new();
    let update_json = serde_json::to_string(&update_data).expect("failed JSON");

    let request = client
        .patch(api_url)
        .headers(headers)
        .body(update_json)
        .send()
        .await
        .expect("Failed to send request");

    if request.status().is_success() {
        println!("GitHub name updated successfully to {}", new_name);
    } else {
        println!("Failed to update GitHub name: {:?}", request.text().await.expect("Failed to get response"));
    }
}

