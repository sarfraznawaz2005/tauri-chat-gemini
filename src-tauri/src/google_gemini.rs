use reqwest::header::{CONTENT_TYPE, HeaderMap};
use serde::{Deserialize, Serialize};
use std::env; // Import the env module to read environment variables
use std::error::Error;

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct RequestPayload {
    contents: Vec<Content>,
}

#[derive(Deserialize)]
struct ResponsePart {
    text: String,
}

#[derive(Deserialize)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

#[derive(Deserialize)]
struct Candidate {
    content: ResponseContent,
}

#[derive(Deserialize)]
struct ApiResponse {
    candidates: Vec<Candidate>,
}

pub async fn generate_content(prompt: String) -> Result<String, Box<dyn Error>> {
    // Read the API key from the environment variable
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not found in environment variables");

    let url = format!("https://generativelanguage.googleapis.com/v1/models/gemini-pro:generateContent?key={}", api_key);

    let payload = RequestPayload {
        contents: vec![
            Content {
                parts: vec![
                    Part { text: prompt },
                ],
            },
        ],
    };

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse()?);

    let res = client.post(&url)
        .headers(headers)
        .json(&payload)
        .send()
        .await?;

    if res.status().is_success() {
        let response: ApiResponse = res.json().await?;
        let mut combined_text = String::new();

        for part in response.candidates.get(0).unwrap().content.parts.iter() {
            combined_text.push_str(&part.text);
            combined_text.push('\n'); // Separating texts with newline for readability
        }

        Ok(combined_text)
    } else {
        Err(Box::new(res.error_for_status().unwrap_err()))
    }
}
