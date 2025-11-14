# Infodisplay Client

The client for my [infodisplay](https://jaroz.ink/posts/infodisplay-v2/), built in [Rust](https://www.rust-lang.org/ "Rust"). It captures screenshots from the [server](https://github.com/c4tz/infodisplay-server)'s website (via [headless_chrome](https://github.com/rust-headless-chrome/rust-headless-chrome)) and displays them on an eInk/ePaper display using a [IT8951 USB driver](https://github.com/faassen/rust-it8951).

It will refresh the picture at every full minute (to show the correct time) and alternate between fast (A2) and full (GC16, every 5th Minute) refresh modes while doing so, in order to create a good compromise between:

* Using only A2 makes the image become darker over time (even when explicitly only refreshing changed pixels/areas)
* A full refresh takes longer a "flashes" the whole screen, which would be quite noticeable when done every minute.

## Configure

The client expects the infodisplay server to be accessible at `http://infodisplay`. You can modify this URL in `src/main.rs` before compiling, if needed.

The target area is automatically initialized based on the display's reported dimensions (tested with 1872×1404 resolution on the 10.3" Waveshare model).

## Build

### On your PC (recommended)

1. Install [cross](https://github.com/cross-rs/cross)

2. (Cross-)Compile for the Rasperry Pi Zero 2 W architecture:
    ```shell
    cross build --target aarch64-unknown-linux-gnu --release
    ```
3. Move the binary to the Pi, e.g. via SCP:
    ```shell
    scp target/aarch64-unknown-linux-gnu/release/infodisplay [RPI-IP]:
    ```

In order to debug locally, you can of course always just plugin the display driver board directly into your PC and build/run the client there:
```shell
cargo build
target/debug/infodisplay
```

### On the raspberry
1. Install Rust on the Raspberry (not needed when compiling on your PC)

2. Build and run:

   ```shell
   cargo build --release
   mv target/release/infodisplay .
   ```

## Run

SSH into your RPI, then just run:
```shell
nohup ./infodisplay &
```

(I might create a systemd unit later on, providing the possbility to run at startup.)

## Disclaimer

I used Rust for this because I have always wanted to do more in this language, but still tried to keep the client as minimal as possible.

I will not implement any feature or change requests that do not create additional value (for me), because this currently works as intended in my view. I will happily accept PRs, though!
