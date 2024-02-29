use reqwest::{Client};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::env;
use serde_json::Value;

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

pub async fn generate_content(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not found in environment variables");
    let url = format!("https://generativelanguage.googleapis.com/v1/models/gemini-pro:generateContent?key={}", api_key);

    let preamble = "You are a helpful and enthusiastic assistant. Use the conversation history provided to inform your responses. If the prompt does not make sense in the context of the conversation history, use your own knowledge to provide an accurate and helpful response.\n\n";
    
    //preamble = "Given the following conversation history, generate a response for the user's last message.\n\n";

    let final_prompt = format!("{}{}", preamble, prompt);

    println!("Prompt sent to AI: \n\"{}\"", final_prompt);
    
    let payload = RequestPayload {
        contents: vec![Content { parts: vec![Part { text: final_prompt }] }],
    };

    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let res = client.post(&url)
        .headers(headers)
        .json(&payload)
        .send()
        .await?;

    if res.status().is_success() {
        let response_body = res.text().await?;
        if response_body.trim().is_empty() {
            // Handle case where response is empty
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "No response received from the API.")));
        }

        let response: ApiResponse = serde_json::from_str(&response_body)?;
        if let Some(candidate) = response.candidates.get(0) {
            let combined_text = candidate.content.parts.iter()
                .map(|part| part.text.clone())
                .collect::<Vec<String>>().join("\n");

            Ok(combined_text)
        } else {
            // Handle case where response is missing expected data
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "No valid response content received from the API.")))
        }
    } else {
        // Parse the error response body
        let error_body = res.text().await?;
        let error_json: Value = serde_json::from_str(&error_body)?;
        let error_message = error_json["error"]["message"].as_str().unwrap_or("Unknown error occurred").to_string();
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_message)))
    }
}




