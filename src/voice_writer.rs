use hound::WavWriter;
use crate::File;
use std::io::BufWriter;

const SAMPLE_RATE: u32 = 23_850; //サンプリングレート 44kHzだとwavファイルへの書き出し速度が3倍ほど早くなってしまう
const CHANNELS: u16 = 2; //チャンネル数(1->モノラル, 2->ステレオ)
const BITS_PER_SAMPLE :u16 = 16; //量子化ビット数

pub fn setup_writer()-> WavWriter<BufWriter<File>> {
    let spec = hound::WavSpec {
        channels: CHANNELS, //チャンネル数(1->モノラル, 2->ステレオ)
        sample_rate: SAMPLE_RATE, //サンプリングレート
        bits_per_sample: BITS_PER_SAMPLE, //量子化ビット数
        sample_format: hound::SampleFormat::Int, //インテジャーPCM
    };
    let writer = hound::WavWriter::create(/*好きなファイルパス名->*/"sample.wav", spec).unwrap();
    return writer
}