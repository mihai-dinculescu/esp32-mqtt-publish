use log::info;

use esp_idf_svc::log::EspLogger;

static LOGGER: EspLogger = EspLogger;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    log::set_logger(&LOGGER).map(|()| LOGGER.initialize())?;
    LOGGER.set_target_level("", log::LevelFilter::Info);

    info!("Hello World! I'm a Rustacean!");

    Ok(())
}
