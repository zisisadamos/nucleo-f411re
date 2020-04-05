#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use nucleo_f411re as board;

use board::hal::{prelude::*, serial::*, stm32};
use cortex_m_rt::entry;
use nb::block;

#[entry]
fn main() -> ! {
    if let Some(p) = stm32::Peripherals::take() {
        let rcc = p.RCC.constrain();
        let clocks = rcc.cfgr.freeze();


        /* Split GPIO pins */
        let gpioa = p.GPIOA.split();
        /* Prepare pins USART2 is connected to */
        let rx = gpioa.pa3.into_alternate_af7();
        let tx = gpioa.pa2.into_alternate_af7();
        let serial_config = config::Config::default();

        /* Setup USART2 which is connected to the on board ST-Link for output */
        let serial_port = Serial::usart2(
            p.USART2,
            (tx, rx),
            serial_config,
            clocks,
        ).unwrap();

        let (mut tx, mut rx) = serial_port.split();

        /* Print a nice hello message */
        let s = b"\r\nPlease type characters to echo:\r\n";

        let _ = s.iter().map(|c| block!(tx.write(*c))).last();

        /* Endless loop */
        loop {
            /* Read and echo back */
            if let Ok(c) = block!(rx.read()) {
                let _ = block!(tx.write(c));
            }
        }
    }

    loop {
        continue;
    }
}
