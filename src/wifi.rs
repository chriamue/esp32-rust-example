use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::timer::EspTaskTimerService;
use esp_idf_svc::wifi::{AsyncWifi, EspWifi};
use esp_idf_sys as _;
use esp_idf_sys::EspError;
use log::info;

const WIFI_SSID: &str = env!("WIFI_SSID");
const WIFI_PASS: &str = env!("WIFI_PASS");

pub struct Wifi<'a> {
    wifi: AsyncWifi<EspWifi<'a>>,
}

impl<'a> Wifi<'a> {
    pub fn new() -> anyhow::Result<Self> {
        let peripherals = Peripherals::take()?;
        let sys_loop = EspSystemEventLoop::take()?;
        let timer_service = EspTaskTimerService::new()?;
        let nvs = EspDefaultNvsPartition::take()?;

        let wifi = AsyncWifi::wrap(
            EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
            sys_loop,
            timer_service,
        )?;
        Ok(Self { wifi })
    }

    pub async fn configure(&mut self) -> Result<(), EspError> {
        info!("Setting Wi-Fi credentials...");

        let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
            ssid: WIFI_SSID.try_into().unwrap(),
            bssid: None,
            auth_method: AuthMethod::WPAWPA2Personal,
            password: WIFI_PASS.try_into().unwrap(),
            channel: None,
        });

        self.wifi.set_configuration(&wifi_configuration)?;

        info!("Starting Wi-Fi driver...");
        self.wifi.start().await
    }

    pub async fn connect(&mut self) -> Result<(), EspError> {
        self.wifi.connect().await?;
        info!("Wifi connected");

        self.wifi.wait_netif_up().await?;
        info!("Wifi netif up");
        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<(), EspError> {
        self.wifi.disconnect().await?;
        info!("Wifi disconnected");
        Ok(())
    }
}
