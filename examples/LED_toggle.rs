#![no_main]
#![no_std]

//Het importeren van de HAL
use hal::prelude::*;
use hal::stm32;
use stm32g473_hal_oppe as hal;

//De code heeft een entry point nodig, en een error handling
use cortex_m_rt::entry;
use panic_semihosting as _;

#[entry]
fn main() -> ! {
    // Gebruik de peripheral crates
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    //Zet de klok op de default mode
    let mut rcc = dp.RCC.constrain();

    //Initialiseer de juiste pinnen
    let gpiob = dp.GPIOB.split(&mut rcc);
    //We gebruiken pin PB7, dit komt overeen met het ledje op de Pyglet. Deze zetten we in output modus
    let mut led = gpiob.pb7.into_push_pull_output();

    let mut delay_syst = cp.SYST.delay(&rcc.clocks);

    loop {
        //Zet de status van de LED, zet een delay, en verander dan de status weer
        led.set_high().unwrap();

        delay_syst.delay_ms(1000);

        led.set_low().unwrap();

        delay_syst.delay_ms(1000);
    }
}
