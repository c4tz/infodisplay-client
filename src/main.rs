use std::error::Error;
use std::thread;

use rust_it8951::{It8951, Mode};
use headless_chrome::{Browser, LaunchOptions};
use headless_chrome::protocol::cdp::Page;
use headless_chrome::protocol::cdp::Emulation;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use image::{RgbImage, load_from_memory, ImageBuffer, DynamicImage};

fn main() -> Result<(), Box<dyn Error>> {

    let mut it8951 = It8951::connect()?;
    let system_info = it8951.get_system_info().unwrap();

    let browser = Browser::new(LaunchOptions {
        window_size: (Some((system_info.width, system_info.height))),
        idle_browser_timeout: Duration::from_secs(600),
        ..Default::default()
    })?;

    let tab = browser.new_tab()?;
    tab.call_method(Emulation::SetDeviceMetricsOverride {
        width: system_info.width,
        height: system_info.height,
        device_scale_factor: 1.0,
        device_posture: None,
        display_feature: None,
        mobile: false,
        scale: None,
        screen_width: None,
        screen_height: None,
        position_x: None,
        position_y: None,
        dont_set_visible_size: None,
        screen_orientation: None,
        viewport: None,
    })?;

    tab.navigate_to("http://infodisplay")?;

    // allow 5s for website to load initially
    thread::sleep(Duration::from_millis(5000));


    let rgb: RgbImage = ImageBuffer::new(system_info.width, system_info.height);
    let canvas: DynamicImage = DynamicImage::ImageRgb8(rgb).grayscale();
    it8951.update_region(&canvas, 0, 0, Mode::INIT)?;
    thread::sleep(Duration::from_millis(1500));

    let mut count = 0;
    loop {
        // take browser screenshot
        let png_data = tab.capture_screenshot(
            Page::CaptureScreenshotFormatOption::Png,
            None,
            None,
            true)?;
        let dyn_img = load_from_memory(&png_data)?;

        // display screenshot on display
        if count >= 5 {
            count = 0;
        }
        let mut mode = Mode::A2;
        if count == 0 {
            mode = Mode::GC16;
        }
        it8951.update_region(&dyn_img.flipv().grayscale(), 0, 0, mode)?;

        // wait until next minute
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards!")
            .as_millis() as u64;

        let current_minute_ms = now_ms % 60_000;
        let next_minute_ms = 60_000 - current_minute_ms;
        let target_ms = next_minute_ms + 1_000;

        thread::sleep(Duration::from_millis(target_ms));
        count += 1;
    }
}
