#![no_main]
#![no_std]

use panic_halt as _;

use stm32f429i_disc as board;

use nb::block;

// use board::stm32f4xx_hal;
use board::stm32f4xx_hal::{prelude::*, pac, serial::{Config, Serial}};

// use core::fmt::Write; 

#[cortex_m_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();

    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(180.MHz()).freeze();

    let mut delay = dp.TIM1.delay_us(&clocks);

    // USART1 at PA9(TX) and PA10(RX) are connected to ST-Link
    let tx = gpioa.pa9.into_alternate::<7>();
    let rx = gpioa.pa10.into_alternate::<7>();

    // Set up USART 1 configured pins and a baudrate of 115200 baud
    let serial: Serial<pac::USART1> = Serial::new(
        dp.USART1,
        (tx, rx),
        Config::default().baudrate(115_200.bps()),
        &clocks,
    )
    .unwrap();

    // Separate out the sender and receiver of the serial port
    let (mut tx, mut rx) = serial.split();
  
    let mut value: u8 = 0x30;

    loop {
        // print some value every 500 ms, value will overflow after 255
        while let Ok(_) = rx.read() {}

        block!(tx.write(value)).ok();

        value += 1;
        if value >= 0x3A { value = 0x30; }

        delay.delay_ms(200);
    }
}
