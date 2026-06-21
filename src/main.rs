use anyhow::{Context, Result, anyhow};
use std::fs;

mod devices;
mod types;
mod utils;

use devices::{DeviceDriver, aula_f75::AulaF75Driver};
use types::{Effect, Key, KeyLayer};

#[derive(serde::Deserialize, serde::Serialize)]
struct KeysWrapper {
    keys: Vec<Key>,
}

fn main() -> Result<()> {
    println!("Loading configuration...");
    let toml_str = fs::read_to_string("test.toml")
        .context("Failed to read test.toml. Make sure it exists.")?;

    let wrapper: KeysWrapper = toml::from_str(&toml_str)
        .context("Failed to parse test.toml")?;
    let default_keys: Vec<Key> = wrapper.keys;

    println!("Scanning for supported devices...");

    let drivers: Vec<Box<dyn DeviceDriver>> = vec![
        Box::new(AulaF75Driver),
    ];

    let api = hidapi::HidApi::new().context("Failed to initialize HID API")?;
    let mut selected_device: Option<Box<dyn devices::Device>> = None;

    for device_info in api.device_list() {
        let vid = device_info.vendor_id();
        let pid = device_info.product_id();

        for driver in &drivers {
            if driver.matches(vid, pid) {
                println!("Found potential device: VID={:#06x} PID={:#06x}", vid, pid);
                match driver.connect(vid, pid) {
                    Ok(dev) => {
                         match dev.get_uuid() {
                             Ok(uuid) => {
                                 println!("Connected! Device UUID: {}", uuid);
                                 selected_device = Some(dev);
                                 break;
                             }
                             Err(e) => {
                                 println!("Failed to get UUID from device: {}", e);
                             }
                         }
                    }
                    Err(e) => {
                        println!("Failed to connect to device: {}", e);
                    }
                }
            }
        }
        if selected_device.is_some() {
            break;
        }
    }

    let device = match selected_device {
        Some(d) => d,
        None => {
            return Err(anyhow!("No supported device found"));
        }
    };

    println!("Writing keymap...");
    device.set_keys(KeyLayer::Normal, &default_keys)?;

    let mut device_info = device.get_basic_info()?;
    device_info.light_mode = Effect::FixedOn;
    device.set_basic_info(&device_info)?;

    device.set_custom_light(&default_keys)?;
    device.set_light_color()?;

    println!("Done.");
    Ok(())
}
