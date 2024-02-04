use anyhow::Ok;
use esp_idf_sys::esp_app_desc;

esp_app_desc!();

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    Ok(())
}
