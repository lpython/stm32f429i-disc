#![no_main]
#![no_std]

use panic_halt as _;

use stm32f429i_disc as board;

use nb::block;


use board::stm32f4xx_hal;
use board::stm32f4xx_hal::{prelude::*, pac, serial::{config::Config, Serial}};


#[cortex_m_rt::entry]
fn main() -> ! {
    if let Some(p) = pac::Peripherals::take() {
        let gpioa = p.GPIOA.split();

        // Constrain clock registers
        let rcc = p.RCC.constrain();

        let clocks = rcc.cfgr.sysclk(180.MHz()).freeze();

        // USART2 at PA9(TX) and PA10(RX) are connected to ST-Link
        let tx = gpioa.pa9.into_alternate::<7>();
        let rx = gpioa.pa10.into_alternate::<7>();

        // Set up USART 1 configured pins and a baudrate of 115200 baud
        let serial: Serial<pac::USART1> = Serial::new(
            p.USART1,
            (tx, rx),
            Config::default().baudrate(115_200.bps()),
            &clocks,
        )
        .unwrap();

        // Separate out the sender and receiver of the serial port
        let (mut tx, mut rx) = serial.split();

        loop {
            // Read character and echo it back
            let received = block!(rx.read()).unwrap();
            block!(tx.write(received)).ok();
        }
    }

    loop {
        continue;
    }
}
