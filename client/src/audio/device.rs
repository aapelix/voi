use cpal::traits::DeviceTrait;

pub fn get_device_name(device: &cpal::Device) -> String {
    let desc = device.description();
    match desc {
        Ok(desc) => desc.name().to_string(),
        Err(_) => "unknown".to_string(),
    }
}
