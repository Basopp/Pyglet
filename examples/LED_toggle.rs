#![no_main]
#![no_std]

use stm32g473_hal_oppe as hal;
use hal::prelude::*;
use hal::stm32;

use cortex_m_rt::entry;
use panic_semihosting as _;

#[entry]
fn main() -> ! {
  
    
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();

    let gpiob = dp.GPIOB.split(&mut rcc);
    let mut led = gpiob.pb7.into_push_pull_output();
    
    let mut delay_syst = cp.SYST.delay(&rcc.clocks);

    loop {
        
        led.set_high().unwrap();
        
        delay_syst.delay_ms(1000);

        led.set_low().unwrap();
        
        delay_syst.delay_ms(1000);
    }
}
