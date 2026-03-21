use cpal::traits::DeviceTrait;
use ringbuf::traits::{Consumer, Observer};

use crate::audio::buffer::AudioBuffer;

pub fn start_output_stream(
    device: &cpal::Device,
    buffer: AudioBuffer,
) -> anyhow::Result<cpal::Stream> {
    let supported = device
        .supported_output_configs()?
        .find(|c| c.sample_format() == cpal::SampleFormat::F32)
        .expect("no supported output config");

    let config = supported.with_max_sample_rate().into();

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _| {
            let mut consumer = buffer.consumer.lock().unwrap();
            for sample in data.iter_mut() {
                if consumer.is_empty() {
                    *sample = 0.0;
                } else {
                    *sample = consumer.try_pop().unwrap_or(0.0);
                }
            }
        },
        move |err| {
            log::error!("stream error: {}", err);
        },
        None,
    )?;

    Ok(stream)
}
