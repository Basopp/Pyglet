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
    let mut count = 0;

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
         //De host moet weten of de device iets wil verzenden. Dit moet (minimaal) om de milliseconde aangeroepen worden
         usb_dev.poll(&mut [&mut serial]);

         count += 1;
         // om wat tijd tussen onze berichten te houden zetten we een counter in, die maar 1 keer in de 200000 clock cycles iets print
         if count == 200000 {
             // Hier zetten we het bericht dat we willen printen. Deze wordt gelijk omgezet naar bytes en dan geprint
             let data = b"Test bericht \n";
             serial.write(data).unwrap();
 
         //counter resetten
         } else if count == 200001 {
             count = 0;
         }
    }
}
