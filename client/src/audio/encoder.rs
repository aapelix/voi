use opus::Encoder;
use ringbuf::traits::Consumer;
use ringbuf::traits::Observer;

use crate::net::PacketSender;
use crate::net::packet::Packet;
use crate::{
    app::{FRAME_SIZE, SAMPLE_RATE},
    audio::buffer::AudioBuffer,
};

pub fn start_encoder(buffer: AudioBuffer, tx: PacketSender) {
    std::thread::spawn(move || {
        let mut encoder = Encoder::new(
            SAMPLE_RATE as u32,
            opus::Channels::Mono,
            opus::Application::Voip,
        )
        .expect("failed to create opus encoder");

        let mut frame = vec![0.0f32; FRAME_SIZE];
        let mut output = vec![0u8; 4000];

        loop {
            if buffer.consumer.lock().unwrap().occupied_len() < FRAME_SIZE {
                std::thread::sleep(std::time::Duration::from_millis(1));
                continue;
            }

            let mut cons = buffer.consumer.lock().unwrap();
            for i in 0..FRAME_SIZE {
                frame[i] = cons.try_pop().unwrap_or(0.0);
            }

            drop(cons);

            let len = encoder.encode_float(&frame, &mut output).unwrap();
            let packet = Packet::Audio(output[..len].to_vec());

            let _ = tx.send(packet);
        }
    });
}
