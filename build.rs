use std::env;

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    embuild::espidf::sysenv::output();

    let wifi_ssid = env::var("WIFI_SSID").unwrap_or_else(|_| "internet".into());
    let wifi_pass = env::var("WIFI_PASS").unwrap_or_else(|_| "password".into());
    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".into());

    println!("cargo:rustc-env=WIFI_SSID={wifi_ssid}");
    println!("cargo:rustc-env=WIFI_PASS={wifi_pass}");
    println!("cargo:rustc-env=SERVER_PORT={server_port}");

    Ok(())
}

// EOF
