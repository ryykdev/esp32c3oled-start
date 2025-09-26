# Rust on ESP32-C3 0.42 OLED dev board - display text and images - MacOS (Apple Silicon)

This repo has some example code you can find using the following tags:
- v1-blinky blink an external LED
- v2-oled-example using the onboard OLED
- v3-uv-sonsor-red using the red GUVA-S12SD sensor


## required setup (apple silicon macos)

1. install rust
2. install https://github.com/esp-rs/espup
	1. run quickstart
    2. source $HOME/export-esp.sh (very important!)
        - to make globally available add to .zshenv - see mine:
        ```.zshenv
        . "$HOME/.cargo/env"
        . "$HOME/export-esp.sh"
        ```
3. install cargo-generate (i needed the vendored-openssl option)
    ```bash
    cargo install cargo-generate --features "vendored-openssl"
    ```
4. template used for this Project: 
    - https://github.com/esp-rs/esp-generate 
    - choose esp32c3 option
    - to finish dialog: with `s` or `S` - bug prevents these to be shown in terminal
5. first time flashing the esp32c3?
    - hold RESET while push and release BOOT (or try reverse order)
6. build and flash device: `cargo run --release`

## hardware
- ESP32-C3 O.42 OLED Dev Board
- MacBook Air M4
