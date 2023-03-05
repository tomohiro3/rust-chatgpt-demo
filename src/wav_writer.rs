use hound::WavWriter;
use crate::File;
use std::io::BufWriter;

const SAMPLE_RATE: f64 = 44_100.0; //サンプリングレート
const CHANNELS: i32 = 2; //チャンネル数(1->モノラル, 2->ステレオ)
// const INTERLEAVED: bool = true; //謎
// const FRAMES_PER_BUFFER: u32 = 64; //フレームごとのバッファ数
const BITS_PER_SAMPLE :u16 = 16; //量子化ビット数
// const NUM_SECONDS: i32 = 10; //録音秒数
// const BUF_SIZE: usize = SAMPLE_RATE as usize * NUM_SECONDS as usize; //保存するバッファサイズ


pub fn setup_writer()-> WavWriter<BufWriter<File>> {
    let spec = hound::WavSpec {
        channels: CHANNELS as u16, //チャンネル数(1->モノラル, 2->ステレオ)
        sample_rate: SAMPLE_RATE as u32, //サンプリングレート
        bits_per_sample: BITS_PER_SAMPLE, //量子化ビット数
        sample_format: hound::SampleFormat::Int, //インテジャーPCM
    };
    let writer = hound::WavWriter::create(/*好きなファイルパス名->*/"sample.wav", spec).unwrap();
    return writer
}