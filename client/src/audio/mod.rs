use std::sync::{Arc, Mutex};

use cpal::{
    Device, Stream,
    traits::{HostTrait, StreamTrait},
};

use crate::audio::{buffer::AudioBuffer, input::start_input_stream, output::start_output_stream};

pub mod buffer;
pub mod decoder;
pub mod device;
pub mod encoder;
mod input;
mod output;

pub struct AudioController {
    pub in_device: Option<Device>,
    pub out_device: Option<Device>,
    pub in_stream: Option<Stream>,
    pub out_stream: Option<Stream>,
}

impl AudioController {
    pub fn new() -> Self {
        Self {
            in_device: None,
            out_device: None,
            in_stream: None,
            out_stream: None,
        }
    }
}

pub type SharedAudio = Arc<Mutex<AudioController>>;

pub fn set_device(
    audio: &SharedAudio,
    capture_buffer: AudioBuffer,
    playback_buffer: AudioBuffer,
    in_device: cpal::Device,
    out_device: cpal::Device,
) -> anyhow::Result<()> {
    let mut audio = audio.lock().unwrap();

    let input_stream = start_input_stream(&in_device, capture_buffer)?;
    let output_stream = start_output_stream(&out_device, playback_buffer)?;

    input_stream.play()?;
    output_stream.play()?;

    audio.in_device = Some(in_device);
    audio.out_device = Some(out_device);
    audio.in_stream = Some(input_stream);
    audio.out_stream = Some(output_stream);

    Ok(())
}

pub fn get_in_devices(host: &cpal::Host) -> anyhow::Result<Vec<Device>> {
    let devices: Vec<Device> = host.input_devices()?.collect();
    Ok(devices)
}

pub fn get_out_devices(host: &cpal::Host) -> anyhow::Result<Vec<Device>> {
    let devices: Vec<Device> = host.output_devices()?.collect();
    Ok(devices)
}
