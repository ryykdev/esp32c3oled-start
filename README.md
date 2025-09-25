# esp32-c3-oled blinkey

This code will blink an LED on GPIO1.

## required setup (apple silicon macos)

1. install rust
2. install https://github.com/esp-rs/espup
	1. run quickstart
    2. source $HOME/export-esp.sh
        - to make globally available add to .zshenv - restart shell
3. install cargo-generate (i needed the vendored-openssl option)
```bash
cargo install cargo-generate --features "vendored-openssl"
```
4. https://github.com/esp-rs/esp-generate
- finish dialog: with `s` or `S` you can generate your project

5. First time flashing the esp32c3? Press-hold RESET while push-release BOOT (worked for me, otherwise try reverse order)

6. `cargo run --release`


