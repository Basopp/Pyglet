
#![no_main]
#![no_std]



use panic_semihosting as _;
use stm32g473_hal_oppe as hal;

use hal::{  stm32, 
            prelude::*,
            adc::{self,
                config::{Continuous, SampleTime, Sequence},
                AdcClaim, ClockSource, Vref
            },
            stm32::Peripherals,
            rcc::{Config, RccExt,SysClockSrc,CK48Src},
            gpio::{GpioExt},
            usb::{Peripheral}};

pub use stm32_usbd::UsbBus;

use cortex_m_rt::entry;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};
use arrform::{arrform, ArrForm};
use core::{sync::atomic::{Ordering, AtomicI32}};
static COUNTER: AtomicI32 = AtomicI32::new(0);
static COUNTER2: AtomicI32 = AtomicI32::new(0);



#[entry]
fn main() -> ! {
  
    
    let cp = cortex_m::Peripherals::take().expect("cannot take core peripherals");
    let dp = stm32::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut rcc = rcc.freeze(Config::new2(SysClockSrc::HSI, CK48Src::HSI48));
    

    let usb = Peripheral {
        usb: dp.USB,
        
    };
    let usb_bus = UsbBus::new(usb);

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .device_class(USB_CLASS_CDC)
        .build();

        let gpioa = dp.GPIOA.split(&mut rcc);
        let pa0 = gpioa.pa0.into_analog();
    
        let mut delay = cp.SYST.delay(&rcc.clocks);
        let mut adc = dp
            .ADC1
            .claim(ClockSource::SystemClock, &rcc, &mut delay, true);
    
       
        adc.enable_vref(&dp.ADC12_COMMON);
        adc.set_auto_delay(true);
        adc.set_continuous(Continuous::Continuous);
        adc.reset_sequence();
        adc.configure_channel(&pa0, Sequence::One, SampleTime::Cycles_640_5);
        adc.configure_channel(&Vref, Sequence::Two, SampleTime::Cycles_640_5);
        
        let adc = adc.enable();
    
      
        let mut adc = adc.start_conversion();
       
    
    loop {

       
       usb_dev.poll(&mut [&mut serial]);
       
    
        let mut state = COUNTER.load(Ordering::Relaxed);
    
        state += 1;

       
        if state == 100000 {
            
            let mut counter_t = COUNTER2.load(Ordering::Relaxed);

            
                if counter_t == 0{
                    adc = adc.wait_for_conversion_sequence().unwrap_active();
                    let millivolts = adc.sample_to_millivolts(adc.current_sample());
                    let tempcalc = millivolts as f32;
                    let mut temperatuur = (tempcalc-2767.0)/(-13.44)-10.0;
                    let mut write_offset = 0;
                    let mut text= arrform!(64, "Temperatuur: {} \n", temperatuur);

                    let x = text.as_bytes();
                    let mut size1 = x.len();
                

                    match serial.write(&x [write_offset..size1]) 
                                 {
                                     Ok(len) if len > 0 => {
                                         write_offset += len;
                                                                 }
                                     _ => {}
                                  }
                    counter_t += 1;
                }

                else if counter_t == 1{
                    adc = adc.wait_for_conversion_sequence().unwrap_active();
                    let millivolts = adc.sample_to_millivolts(adc.current_sample());
                    let mut write_offset = 0;
                    let mut text= arrform!(64, "Vref: {} \n", millivolts);

                    let x = text.as_bytes();
                    let mut size1 = x.len();
                

                    match serial.write(&x [write_offset..size1]) 
                                 {
                                     Ok(len) if len > 0 => {
                                         write_offset += len;
                                                                 }
                                     _ => {}
                                  }
                    counter_t += 1;
                }
                else if counter_t == 2 {counter_t = 0;}

                COUNTER2.store(counter_t,Ordering::Relaxed);
                    
        }
        else if state == 100001{
            state = 0;
        }
    
       COUNTER.store(state,Ordering::Relaxed);
      
    }
}
