#![no_std]
#![no_main]

use panic_semihosting as _;
use stm32g473_hal_oppe as hal;
use cortex_m_rt::entry;

use hal::{rcc::{Config, RccExt,SysClockSrc,CK48Src},stm32,prelude::* };
use hal::usb::{Peripheral};
pub use stm32_usbd::UsbBus; 

use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};


#[entry]
fn main() -> ! {
   
    let dp = stm32::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    // Hier stellen we de klok op een eigen instelling in, we gebruiken de standaard interne klok (16Mhz), maar ook de USB klok (48Mhz)
    let mut rcc = rcc.freeze(Config::new2(SysClockSrc::HSI, CK48Src::HSI48));
    let mut count = 0;
    
    let usb = Peripheral {
        usb: dp.USB,
        
    };
    let usb_bus = UsbBus::new(usb);

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Aemics")
        .device_class(USB_CLASS_CDC)
        .build();

    loop {
        
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
               

                // bit veranderen zodat het in hoofdletters terug echo't
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

                let mut write_offset = 0;
                while write_offset < count {
                    match serial.write(&buf[write_offset..count]) {
                        Ok(len) if len > 0 => {
                            write_offset += len;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        
    }

}

