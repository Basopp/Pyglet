#![no_std]
#![no_main]

use panic_semihosting as _;
use stm32g473_hal_oppe as hal;

use hal::usb::Peripheral;
use hal::{
    rcc::{CK48Src, Config, RccExt, SysClockSrc},
    stm32,
};
pub use stm32_usbd::UsbBus;

use cortex_m_rt::entry;

//USB drivers
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    
    // Hier stellen we de klok op een eigen instelling in, we gebruiken de standaard interne klok (16Mhz), maar ook de USB klok (48Mhz)
    dp.RCC.freeze(Config::new2(SysClockSrc::HSI, CK48Src::HSI48));
    

    //USB peripheral gebruiken
    let usb = Peripheral { usb: dp.USB };
    let usb_bus = UsbBus::new(usb);

    //seriele poort aanmaken
    let mut serial = SerialPort::new(&usb_bus);

    //USB instellingen en de device aanmaken
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .device_class(USB_CLASS_CDC)
        .build();

    loop {

        usb_dev.poll(&mut [&mut serial]);

        //buffer aanmaken voor de karakters die we gaan typen
        let mut buf = [0u8; 64];

        //inkomende data lezen
        match serial.read(&mut buf) {
            Ok(_) => {
                // bit veranderen zodat het in hoofdletters terug echo't
                for c in buf[0..64].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }
            
            //aangepaste data terugsturen
            serial.write(&buf).unwrap();
             
            }
        _ => {}
        }
    }
}
