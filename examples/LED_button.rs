#![no_main]
#![no_std]

use panic_semihosting as _;


use stm32g473_hal_oppe::{
    
    prelude::*,
    gpio::{gpiob, ExtiPin, GpioExt, Input, PullDown, SignalEdge},
    rcc::RccExt,
    rcc::Config,
    stm32,
    stm32::{Interrupt,interrupt},
    syscfg::SysCfgExt,
   
};

use core::cell::RefCell;

use cortex_m::{asm::wfi, interrupt::Mutex};
use cortex_m_rt::entry;


use core::sync::atomic::{AtomicU8, Ordering};
type ButtonPin = gpiob::PB10<Input<PullDown>>;

static G_BUTTON: Mutex<RefCell<Option<ButtonPin>>> = Mutex::new(RefCell::new(None));
static G_LED_STATE: AtomicU8 = AtomicU8::new(0);


// Define an interupt handler, i.e. function to call when interrupt occurs.
// This specific interrupt will "trip" when the button is pressed
#[interrupt]
unsafe fn EXTI15_10() {
    static mut BUTTON: Option<ButtonPin> = None;


    let button = BUTTON.get_or_insert_with(|| {
        
        cortex_m::interrupt::free(|cs| {
            
            G_BUTTON.borrow(cs).replace(None).unwrap()
        })
    });

    let mut state = G_LED_STATE.load(Ordering::Relaxed);
    state += 1;
    if state == 2 {
        state = 0;
    }
        
    G_LED_STATE.store(state,(Ordering::Relaxed));
    button.clear_interrupt_pending_bit();
}

#[entry]
fn main() -> ! {


    let mut dp = stm32::Peripherals::take().unwrap();

    
    let mut rcc = dp.RCC.freeze(Config::hsi());
    let mut syscfg = dp.SYSCFG.constrain();
    

    // Configure PB7 pin to blink LED
    let gpiob = dp.GPIOB.split(&mut rcc);
    let mut led = gpiob.pb7.into_push_pull_output();

   
    let mut button = gpiob.pb10.into_pull_down_input();
    button.make_interrupt_source(&mut syscfg);
    button.trigger_on_edge(&mut dp.EXTI, SignalEdge::Rising);
    button.enable_interrupt(&mut dp.EXTI);

    cortex_m::interrupt::free(|cs| *G_BUTTON.borrow(cs).borrow_mut() = Some(button));

    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupt::EXTI15_10);
    }

    
    
    loop {
        wfi();
        
        let mut state = G_LED_STATE.load(Ordering::Relaxed);
        if state == 0 {
            led.set_high().unwrap();
        }
        else if state == 1  {
           
            led.set_low().unwrap();
        } 
            
     
    }
}
