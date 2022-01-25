use anyhow::bail;
use log::info;
use std::sync::Arc;

use embedded_svc::wifi::*;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::netif::EspNetifStack;
use esp_idf_svc::nvs::EspDefaultNvs;
use esp_idf_svc::sysloop::EspSysLoopStack;
use esp_idf_svc::wifi::EspWifi;

static LOGGER: EspLogger = EspLogger;

// !!! SET THIS !!!
const WIFI_SSID: &str = "";
const WIFI_PASS: &str = "";

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    log::set_logger(&LOGGER).map(|()| LOGGER.initialize())?;
    LOGGER.set_target_level("", log::LevelFilter::Info);

    let netif_stack = Arc::new(EspNetifStack::new()?);
    let sys_loop_stack = Arc::new(EspSysLoopStack::new()?);
    let default_nvs = Arc::new(EspDefaultNvs::new()?);

    let _wifi = setup_wifi(
        netif_stack,
        sys_loop_stack,
        default_nvs,
        WIFI_SSID,
        WIFI_PASS,
    )?;

    Ok(())
}

fn setup_wifi(
    netif_stack: Arc<EspNetifStack>,
    sys_loop_stack: Arc<EspSysLoopStack>,
    default_nvs: Arc<EspDefaultNvs>,
    ssid: &str,
    password: &str,
) -> anyhow::Result<Box<EspWifi>> {
    let mut wifi = Box::new(EspWifi::new(netif_stack, sys_loop_stack, default_nvs)?);

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: ssid.into(),
        password: password.into(),
        ..Default::default()
    }))?;

    info!("Wifi configuration set, about to get status");

    let status = wifi.get_status();

    if let Status(
        ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(_))),
        _,
    ) = status
    {
        info!("Wifi connected");
    } else {
        bail!("Unexpected Wifi status: {:?}", status);
    }

    Ok(wifi)
}
