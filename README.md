# esp32-rust-example

This is a simple example of how to use Rust to program an ESP32 microcontroller.

This project is based on the [esp-idf-template](https://github.com/esp-rs/esp-idf-template) project.

## Prerequisites

See [Prequisites](https://github.com/esp-rs/esp-idf-template?tab=readme-ov-file#prerequisites)

Add user to dialout group to access serial port:

```bash
sudo usermod -a -G dialout $USER
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

## Building

```bash
. $HOME/export-esp.sh
cargo build
```

## Flashing

```bash
cargo espflash monitor
```
