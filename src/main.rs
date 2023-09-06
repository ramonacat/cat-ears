//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bsp::{
    entry,
    hal::{prelude::*, Timer},
};
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};
use ws2812_pio::Ws2812;

use smart_leds_trait::{SmartLedsWrite, RGB8};

#[cfg(not(test))]
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(peripherals.WATCHDOG);
    let sio = Sio::new(peripherals.SIO);

    let mut clocks = init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        peripherals.XOSC,
        peripherals.CLOCKS,
        peripherals.PLL_SYS,
        peripherals.PLL_USB,
        &mut peripherals.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    let timer = Timer::new(peripherals.TIMER, &mut peripherals.RESETS, &clocks);

    let pins = rp_pico::Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS,
    );

    let (mut pio, sm0, _, _, _) = peripherals.PIO0.split(&mut peripherals.RESETS);
    let mut ws = Ws2812::new(
        pins.gpio4.into_mode(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    let core = pac::CorePeripherals::take().unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().raw());

    const color: RGB8 = RGB8::new(0xff, 0x18, 0x85);
    let mut data: [RGB8; 70] = [color; 70];
    let mut flip = false;

    loop {
        use smart_leds::{SmartLedsWrite, RGB8};

        let range = 0..35;
        for mut i in  range {
            for item in data.iter_mut() {
                *item = RGB8::default();
            }

            if flip {
                i = 35 - i - 1;
            }

            data[i] = color;
            data[70 - i - 1] = color;

            ws.write(data.iter().copied()).unwrap();
            delay.delay_ms(100);
        }

        flip = !flip;
    }
}
