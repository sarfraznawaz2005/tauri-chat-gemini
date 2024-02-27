use reqwest::{Client};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::env;

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

    // Define the preamble
    let preamble = "You are a helpful and enthusiastic assistant. Use the conversation history provided to inform your responses. If the prompt does not make sense in the context of the conversation history, use your own knowledge to provide an accurate and helpful response.\n\n";

    // Prepend the preamble to the prompt
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
        let response: ApiResponse = res.json().await?;
        let combined_text = response.candidates.get(0).unwrap().content.parts.iter()
            .map(|part| part.text.clone())
            .collect::<Vec<String>>().join("\n");

        Ok(combined_text)
    } else {
        Err(Box::new(res.error_for_status().unwrap_err()))
    }
}



