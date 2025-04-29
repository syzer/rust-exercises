#![no_main]
#![no_std]

use core::time::Duration;

use cortex_m_rt::entry;
// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use hal_app as _;

const POLL_INTERVAL: Duration = Duration::from_millis(500);

#[entry]
fn main() -> ! {
    let board = dk::init().unwrap();

    let mut leds = [
        board.leds._1,
        board.leds._2,
        board.leds._3,
        board.leds._4,
    ];

    let mut timer = board.timer;

    let ms = POLL_INTERVAL.as_millis();
    defmt::println!("Polling button every {}ms", ms);

    // let mut button = board.buttons._1;
    loop {
        // let is_pressed = button.is_pressed();
        let is_pressed = true; //button.is_pressed();

        if is_pressed {

            for led in leds.iter_mut() {
                led.on();
            }

            timer.wait(POLL_INTERVAL);

            for led in leds.iter_mut() {
                timer.wait(POLL_INTERVAL);

                led.off();
            }

        } else {
            for led in leds.iter_mut() {
                led.off();
            }
        }
        timer.wait(POLL_INTERVAL);
    }
}
