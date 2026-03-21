use eframe::egui;
use std::sync::{Arc, Mutex};

use crate::audio::{
    AudioController, SharedAudio, buffer::AudioBuffer, device::get_device_name, get_in_devices,
    get_out_devices,
};

pub struct Application {
    audio: SharedAudio,
    buffer: AudioBuffer,
    in_devices: Vec<cpal::Device>,
    in_device_names: Vec<String>,
    out_devices: Vec<cpal::Device>,
    out_device_names: Vec<String>,
    in_selected: usize,
    out_selected: usize,
}

impl Application {
    pub fn new() -> Self {
        let host = Arc::new(cpal::default_host());
        let audio = Arc::new(Mutex::new(AudioController::new()));
        let buffer = AudioBuffer::new(44100 * 10); // 10 seconds of audio at 44.1kHz

        let in_devices = get_in_devices(&host).unwrap_or_default();
        let in_device_names = in_devices.iter().map(|d| get_device_name(d)).collect();

        let out_devices = get_out_devices(&host).unwrap_or_default();
        let out_device_names = out_devices.iter().map(|d| get_device_name(d)).collect();

        Self {
            audio,
            buffer,
            in_devices,
            in_device_names,
            out_devices,
            out_device_names,
            in_selected: 0,
            out_selected: 0,
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let dt = ctx.input(|i| i.stable_dt);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Audio Devices");
            ui.label(format!("dt: {:.4} sec", dt));

            egui::ComboBox::from_label("Input device")
                .selected_text(
                    self.in_device_names
                        .get(self.in_selected)
                        .cloned()
                        .unwrap_or_else(|| "unknown".to_string()),
                )
                .show_ui(ui, |ui| {
                    for (i, name) in self.in_device_names.iter().enumerate() {
                        ui.selectable_value(&mut self.in_selected, i, name);
                    }
                });

            egui::ComboBox::from_label("Output device")
                .selected_text(
                    self.out_device_names
                        .get(self.out_selected)
                        .cloned()
                        .unwrap_or_else(|| "unknown".to_string()),
                )
                .show_ui(ui, |ui| {
                    for (i, name) in self.out_device_names.iter().enumerate() {
                        ui.selectable_value(&mut self.out_selected, i, name);
                    }
                });

            if ui.button("Save").clicked() {
                let in_device = self.in_devices[self.in_selected].clone();
                let out_device = self.out_devices[self.out_selected].clone();
                let buffer = self.buffer.clone();
                let audio = self.audio.clone();

                std::thread::spawn(move || {
                    let _ = crate::audio::set_device(&audio, buffer, in_device, out_device);
                });
            }
        });
    }
}
