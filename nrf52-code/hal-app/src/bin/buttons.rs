#![no_main]
#![no_std]

use core::time::Duration;

use cortex_m_rt::entry;
// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use hal_app as _;

const POLL_INTERVAL: Duration = Duration::from_millis(1000);

#[entry]
fn main() -> ! {
    let board = dk::init().unwrap();

    let mut led = board.leds._1;
    let mut timer = board.timer;
    // Uncomment the line below
    // 👇

    let ms = POLL_INTERVAL.as_millis();
    defmt::println!("Polling button every {}ms", ms);
    loop {
        // 👇
        if true {
            led.on();
            timer.wait(POLL_INTERVAL);
            led.off();
        } else {
            led.off();
        }
        timer.wait(POLL_INTERVAL);
    }
}
