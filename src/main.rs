use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::{time,thread};
use std::fs::File;
use hound;
use reqwest::{header::HeaderMap,blocking::Client, blocking::multipart::Form};

use std::io::Read;

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

const SAMPLE_RATE: f64 = 44_100.0; //サンプリングレート
const CHANNELS: i32 = 2; //チャンネル数(1->モノラル, 2->ステレオ)
// const INTERLEAVED: bool = true; //謎
// const FRAMES_PER_BUFFER: u32 = 64; //フレームごとのバッファ数
const BITS_PER_SAMPLE :u16 = 16; //量子化ビット数
// const NUM_SECONDS: i32 = 10; //録音秒数
// const BUF_SIZE: usize = SAMPLE_RATE as usize * NUM_SECONDS as usize; //保存するバッファサイズ


fn main() {
    let spec = hound::WavSpec {
        channels: CHANNELS as u16, //チャンネル数(1->モノラル, 2->ステレオ)
        sample_rate: SAMPLE_RATE as u32, //サンプリングレート
        bits_per_sample: BITS_PER_SAMPLE, //量子化ビット数
        sample_format: hound::SampleFormat::Int, //インテジャーPCM
    };
    let mut writer = hound::WavWriter::create(/*好きなファイルパス名->*/"sample.wav", spec).unwrap();

    let host = cpal::default_host();
    let device = host.default_input_device().expect("Failed to get default input device");
    
    let config = device
    .default_input_config()
    .expect("No default input config")
    .config();

    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

    let stream = device.build_input_stream(
            &config.into(),
            move|data:&[f32], _: &_| {

                for sample in data.iter() {
                    let amplitude = i16::MAX as f32;
                    writer.write_sample((sample * amplitude) as i16).unwrap(); //書き出し
                }
            },
            err_fn,
    None
        ).expect("Stream ended");
    stream.play().expect("Failed to start stream");

    let three_sec = time::Duration::from_millis(3000);
        thread::sleep(three_sec);
    stream.pause().expect("failed");

    const API_KEY:&str = "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";
    let client = Client::new();
    let mut headers =HeaderMap::new();
    let token = format!("Bearer {API_KEY}");
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
