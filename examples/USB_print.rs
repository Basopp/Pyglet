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
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(USB_CLASS_CDC)
        .build();

    loop {
        
    usb_dev.poll(&mut [&mut serial]);
    
    count +=1; 
    // om wat tijd tussen onze berichten te houden zetten we een counter in, die maar 1 keer in de 200000 clock cycles iets print
    if count == 200000 {

    // Hier zetten we het bericht dat we wilen printen. Deze wordt gelijk omgezet naar bytes en de lengte wordt berekent
    let mut x = b"Test bericht \n" ;
    let mut size = x.len();
   
    let mut write_offset = 0;
  
    match serial.write(&x[write_offset..size]) {
                        Ok(len) if len > 0 => {
                            write_offset += len;
                        }
                        _ => {}
                 } 
        }
        else if count == 200001{
            count = 0;
        }
        
    }
}
