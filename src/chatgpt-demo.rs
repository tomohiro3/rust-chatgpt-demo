use reqwest::{blocking::Client, header::HeaderMap};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    const API_KEY:&str = "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";

    let client = Client::new();
    let mut headers =HeaderMap::new();
    let token = format!("Bearer {API_KEY}");
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Authorization", token.parse().unwrap());

    let prompt = "What is engineering";
    let data = format!("{{\"prompt\": \"{}\", \"temperature\": 0.7, \"max_tokens\": 50}}", prompt);

    let response = client.post("https://api.openai.com/v1/engines/davinci-codex/completions")
        .headers(headers)
        .body(data)
        .send()?
        .text()?;

    println!("{}", response);

    Ok(())
    
}