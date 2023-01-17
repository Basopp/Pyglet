#![no_main]
#![no_std]

use panic_semihosting as _;

//Alle onderdelen uit de HAL die we nodig hebben
use stm32g473_hal_oppe::{
    gpio::{gpiob, ExtiPin, GpioExt, Input, PullDown, SignalEdge},
    prelude::*,
    rcc::Config,
    rcc::RccExt,
    stm32,
    stm32::{interrupt, Interrupt},
    syscfg::SysCfgExt,
};

//Zorgt ervoor dat we interrupts kunnen gebruiken
use cortex_m::{asm::wfi, interrupt::Mutex};
use cortex_m_rt::entry;

use core::cell::RefCell;
use core::sync::atomic::{AtomicU8, Ordering};
type ButtonPin = gpiob::PB10<Input<PullDown>>;

//Zorgt ervoor dat de status van de LED en de button tussen de interrupt en het main programma gelezen kunnen worden
static G_BUTTON: Mutex<RefCell<Option<ButtonPin>>> = Mutex::new(RefCell::new(None));
static G_LED_STATE: AtomicU8 = AtomicU8::new(0);

//Dit keer gebruiken we een ander soort interrupt, namelijk de external interrupt
#[interrupt]
unsafe fn EXTI15_10() {
    static mut BUTTON: Option<ButtonPin> = None;

    //laadt de button in de interrupt
    let button = BUTTON.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| G_BUTTON.borrow(cs).replace(None).unwrap())
    });

    //Verander de status van de variabele die bijhoudt of de LED aan of uit is
    let mut state = G_LED_STATE.load(Ordering::Relaxed);
    state += 1;
    if state == 2 {
        state = 0;
    }
    //Sla de nieuwe status van de LED op
    G_LED_STATE.store(state, Ordering::Relaxed);
    button.clear_interrupt_pending_bit();
}

#[entry]
fn main() -> ! {
    let mut dp = stm32::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(Config::hsi());
    let mut syscfg = dp.SYSCFG.constrain();

    let gpiob = dp.GPIOB.split(&mut rcc);
    let mut led = gpiob.pb7.into_push_pull_output();

    // Button instellen zodat hij de interrupt aanmaakt
    let mut button = gpiob.pb10.into_pull_down_input();
    button.make_interrupt_source(&mut syscfg);
    button.trigger_on_edge(&mut dp.EXTI, SignalEdge::Rising);
    button.enable_interrupt(&mut dp.EXTI);

    cortex_m::interrupt::free(|cs| *G_BUTTON.borrow(cs).borrow_mut() = Some(button));

    //interrupt enable
    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupt::EXTI15_10);
    }

    loop {
        wfi();
        
        //status inladen
        let state = G_LED_STATE.load(Ordering::Relaxed);
        if state == 0 {
            led.set_high().unwrap();
        } else if state == 1 {
            led.set_low().unwrap();
        }
    }
}
