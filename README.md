# esp32-rust-example

This is a simple example of how to use Rust to program an ESP32 microcontroller.

This project is based on the [esp-idf-template](https://github.com/esp-rs/esp-idf-template) project.

## Prerequisites

See [Prequisites](https://github.com/esp-rs/esp-idf-template?tab=readme-ov-file#prerequisites)

Add user to dialout group to access serial port:

```bash
sudo usermod -a -G dialout $USER
```

Install cargo espflash:

```bash
cargo install cargo-espflash
```

## Board Information

```bash
cargo espflash board-info
```

outputs:

```bash
[2024-02-04T12:12:44Z INFO ] Serial port: '/dev/ttyUSB0'
[2024-02-04T12:12:44Z INFO ] Connecting...
[2024-02-04T12:12:45Z INFO ] Unable to connect, retrying with extra delay...
[2024-02-04T12:12:46Z INFO ] Using flash stub
Chip type:         esp32 (revision v1.0)
Crystal frequency: 40MHz
Flash size:        4MB
Features:          WiFi, BT, Dual Core, 240MHz, Coding Scheme None
MAC address:       24:6f:28:00:00:00
```

### Troubleshooting

If you get an error like:

```bash
[2024-02-04T12:12:44Z INFO ] Serial port: '/dev/ttyUSB0'
[2024-02-04T12:12:44Z INFO ] Connecting...
[2024-02-04T12:12:45Z INFO ] Unable to connect, retrying with extra delay...
```

Try to press the boot button on the ESP32 board and run the command again.

## Setup

Copy `example.env` to `.env` and adjust the settings.

## Building

```bash
. $HOME/export-esp.sh
cargo build
```

### Features

I tested on two boards:

- [Heltec WiFi LoRa 32 V2](https://resource.heltec.cn/download/Manual%20Old/WiFi%20Lora32Manual.pdf)
- [Heltec WiFi LoRa 32 V3](https://heltec.org/project/wifi-lora-32-v3/)

To configure the right pins, you can use the `v2` or `v3` feature:

```bash
cargo build --features v2
```

## Flashing

```bash
cargo espflash flash --release --monitor --partition-table partitions.csv
```

## License

[MIT](LICENSE)

## See also

- [Tokio running on esp32!](https://github.com/jasta/esp32-tokio-demo)
- [IRC bot with JSON API for esp32](https://github.com/sjm42/esp32ircbot)
- [ESP (Ikea) Vindriktning & Rust 🦀](https://github.com/vojty/ESP-Vindriktning/blob/main/src/wifi.rs)