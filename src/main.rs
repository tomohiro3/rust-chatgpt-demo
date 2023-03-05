use std::{time,thread};
use cpal::traits::StreamTrait;
use std::fs::File;
use reqwest::{header::HeaderMap,blocking::Client, blocking::multipart::Form};
use dotenvy::dotenv;
use std::env;
use std::io::Read;

mod stream;
mod wav_writer;

#[derive(serde::Deserialize)]
struct TranscriptionResponse {
    text: String,
}

#[derive(serde::Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(serde::Deserialize)]
struct Choice {
    message: Content,
}
#[derive(serde::Deserialize)]
struct Content {
    content: String,
}


// モジュール分けると型とか諸々依存関係強くなる。
// 関数型言語のお作法を知る必要あり。カプセル化とかするのか。
// コールバックで渡したい。
fn main() {
    dotenv().ok();

    let writer = wav_writer::setup_writer();
    let stream = stream::setup_stream(writer);
    
    stream.play().expect("Failed to start stream");

    let three_sec = time::Duration::from_millis(3000);
        thread::sleep(three_sec);
    stream.pause().expect("failed");



    let api_key = env::var("OPENAI_API_KEY").expect("EMAIL_BACKEND not found");
    let client = Client::new();
    let mut headers =HeaderMap::new();
    let token = format!("Bearer {api_key}");
    headers.insert("Authorization", token.parse().unwrap());


    

    let mut file = File::open("sample.wav").expect("Failed");
    let mut file_contents = vec![];
    file.read_to_end(&mut file_contents).expect("error");

    let form = Form::new()
    .text("model", "whisper-1")
    .part("file", reqwest::blocking::multipart::Part::bytes(file_contents).file_name("sample.wav"));

    let response: TranscriptionResponse = client.post("https://api.openai.com/v1/audio/transcriptions")
    .headers(headers)
    .multipart(form)
    .send().expect("Failed")
    .json().expect("failed");
    



    let client2 = Client::new();
    let mut headers2 =HeaderMap::new();
    headers2.insert("Content-Type", "application/json".parse().unwrap());
    headers2.insert("Authorization", token.parse().unwrap());

    let data = format!("{{\"model\": \"gpt-3.5-turbo\", \"messages\": [{{\"role\": \"user\", \"content\": \"{}\"}}]}}", response.text);
    let response:ChatResponse = client2.post("https://api.openai.com/v1/chat/completions")
        .headers(headers2)
        .body(data)
        .send().expect("error")
        .json().expect("eorr");

    println!("{}", response.choices[0].message.content);

}
