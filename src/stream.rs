use std::f32::consts::PI;
use cpal::traits::{DeviceTrait, HostTrait};
use hound::WavWriter;
use crate::File;
use std::io::BufWriter;
use cpal::Stream;

pub fn setup_stream(mut writer:WavWriter<BufWriter<File>>)->Stream {
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

                for t in data.iter() {
                    let sample = t;
                    let amplitude = i16::MAX as f32; // 音の大きさを増幅させる
                    writer.write_sample((sample * amplitude) as i16).unwrap(); //書き出し
                }
            },
            err_fn,
    None
        ).expect("Stream ended");

    return stream;
}