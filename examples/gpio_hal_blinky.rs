#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use nucleo_f411re as board;

use board::hal::{prelude::*, stm32};
use cortex_m_rt::entry;


use stm32f4xx_hal::delay::Delay;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let syst = cp.SYST;

    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze();

    let gpioa = dp.GPIOA.split();

    let mut led = gpioa.pa5.into_push_pull_output();

    let mut delay_provider = Delay::new(syst, clocks);

    loop {
        delay_provider.delay_ms(2000 as u32);
        let _ = led.set_high();
        delay_provider.delay_ms(2000 as u32);
        let _ = led.set_low();
    }
}
