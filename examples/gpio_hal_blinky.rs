#![no_main]
#![no_std]

use panic_halt as _;

use stm32f429i_disc as board;

use cortex_m_rt::entry;

use board::stm32f4xx_hal::prelude::*;
use board::stm32f4xx_hal::pac;

// use board::led::{Color, Leds};

use cortex_m::peripheral::Peripherals;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (pac::Peripherals::take(), Peripherals::take()) {
        let gpiog = p.GPIOG.split();

        // (Re-)configure PG13 (green LED) as output
        let mut led = gpiog.pg13.into_push_pull_output();

        // Constrain clock registers
        let rcc = p.RCC.constrain();

        // Configure clock to 180 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(180.MHz()).freeze();

        // Get delay provider
        // let mut delay = Delay::new(cp.SYST, clocks);

        let mut delay = cp.SYST.delay(&clocks);
        
        loop {
            // Toggle LED
            led.toggle();

            // Delay a second
            delay.delay_ms(1000);
        }
    }

    loop {
        continue;
    }
}
