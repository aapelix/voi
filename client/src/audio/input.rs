use cpal::traits::DeviceTrait;
use ringbuf::traits::{Observer, Producer};

use crate::audio::buffer::AudioBuffer;

pub fn start_input_stream(
    device: &cpal::Device,
    buffer: AudioBuffer,
) -> anyhow::Result<cpal::Stream> {
    let supported = device
        .supported_input_configs()?
        .find(|c| c.sample_format() == cpal::SampleFormat::F32)
        .expect("no supported input config");

    let config = supported.with_max_sample_rate().into();

    let stream = device.build_input_stream(
        &config,
        move |data: &[f32], _| {
            let mut prod = buffer.producer.lock().unwrap();
            for &sample in data {
                if prod.is_full() {
                    break;
                }
                prod.try_push(sample).ok();
            }
        },
        move |err| {
            log::error!("stream error: {}", err);
        },
        None,
    )?;

    Ok(stream)
}
