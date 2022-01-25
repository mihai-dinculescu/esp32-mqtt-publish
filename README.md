# ESP32 MQTT Publish with Rust

Accompanying code for [https://medium.com/iotics/esp32-mqtt-publish-with-rust-678d1068ee2](https://medium.com/iotics/esp32-mqtt-publish-with-rust-678d1068ee2).

# Find out the serial port

```bash
espflash board-info
```

# Build, Flash & Monitor

```bash
cargo build
espflash <SERIAL-PORT> target/xtensa-esp32-espidf/debug/esp32_mqtt_publish
espmonitor <SERIAL-PORT>
```
