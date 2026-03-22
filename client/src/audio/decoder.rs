use opus::Decoder;
use ringbuf::traits::Producer;

use crate::{
    app::{FRAME_SIZE, SAMPLE_RATE},
    audio::buffer::AudioBuffer,
    net::{PacketReceiver, packet::Packet},
};

pub fn start_decoder(mut rx: PacketReceiver, buffer: AudioBuffer) {
    std::thread::spawn(move || {
        let mut decoder = Decoder::new(SAMPLE_RATE as u32, opus::Channels::Mono)
            .expect("failed to create opus decoder");

        let mut output = vec![0.0f32; FRAME_SIZE];

        while let Some(packet) = rx.blocking_recv() {
            match packet {
                Packet::Audio(data) => {
                    if let Ok(samples) = decoder.decode_float(&data, &mut output, false) {
                        let mut prod = buffer.producer.lock().unwrap();

                        for i in 0..samples {
                            let _ = prod.try_push(output[i]);
                        }
                    }
                }
            }
        }
    });
}
