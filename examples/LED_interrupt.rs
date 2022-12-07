#![no_main]
#![no_std]

use panic_semihosting as _;
use stm32g473_hal_oppe as hal;

use hal::{  stm32::{interrupt,Interrupt,self}, 
            prelude::*, 
            
            gpio::GpioExt, 
            rcc::{Config, RccExt},
            timer::{Timer,Event, CountDownTimer},
        };

use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::{asm::wfi, interrupt::Mutex};
use cortex_m_rt::entry;

static TIMER_TIM2: Mutex<RefCell<Option<CountDownTimer<stm32::TIM2>>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn TIM2() {
   
 
    cortex_m::interrupt::free(|cs| {
            if let Some(ref mut t2) = TIMER_TIM2.borrow(cs).borrow_mut().deref_mut() {
                t2.clear_interrupt(Event::TimeOut);
                t2.start(100.ms())
            }
        });
 
}


#[entry]
fn main() -> ! {
  

    
    let dp = stm32::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.freeze(Config::hsi());

    let gpiob = dp.GPIOB.split(&mut rcc);
    let mut led = gpiob.pb7.into_push_pull_output();
    
    let timer2 = Timer::new(dp.TIM2, &rcc.clocks);
    
    let mut timer2_i = timer2.start_count_down(100.ms());
    timer2_i.listen(Event::TimeOut);

    cortex_m::interrupt::free(|cs| TIMER_TIM2.borrow(cs).borrow_mut().replace(timer2_i));


    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupt::TIM2);
        
    }
    
    loop {
        wfi();
        led.toggle().unwrap();
      
    }
}
