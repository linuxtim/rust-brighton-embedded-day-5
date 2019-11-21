#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m;
use cortex_m_rt::entry;

use hal::spi::*;

use crate::hal::{prelude::*, stm32};
use stm32f4xx_hal as hal;

use ws2812_spi as ws2812;
use crate::ws2812::Ws2812;
use smart_leds_trait::RGB8;
use smart_leds::SmartLedsWrite;


// NOTE:
// NOTE: Red Power (3.3v to 5v)
// NOTE: Green data MoSi D4
// NOTE: **** White GND ****
// NOTE:


#[entry]
fn main() -> ! {
    // Access the device peripherals (dp) and cortex peripherals (cp):
    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED: it's connected to pin PA5 on the microcontroler
/*        let gpioa = dp.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();

        // The external LED, on the next pin down:
        let mut xled = gpioa.pa6.into_push_pull_output();
*/

        // We use the SPI1 peripheral - we use some GPIO pins, and then
        // change them into an SPI peripheral (this is "alternate function 5") 
        let gpiob = dp.GPIOB.split();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Configure pins for SPI
        // We don't connect sck, but I think the SPI traits require it?
        let sck = gpiob.pb3.into_alternate_af5();
        // Master Out Slave In - pb5, Nucleo 64 pin d4
        let mosi = gpiob.pb5.into_alternate_af5();

        let spi = Spi::spi1(
            dp.SPI1,
            (sck, NoMiso, mosi),
            Mode {
                polarity: Polarity::IdleLow,
                phase: Phase::CaptureOnFirstTransition,
            },
            stm32f4xx_hal::time::KiloHertz(3000).into(),
            clocks,
        );

        // Create a delay abstraction based on SysTick
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);


        let mut data: [RGB8; 4] = [RGB8::default(); 4];
        let empty: [RGB8; 4] = [RGB8::default(); 4];
        let mut ws = Ws2812::new(spi);
        loop {
            data[0] = RGB8 {
                r: 0,
                g: 0,
                b: 0xa0,
            };
            data[1] = RGB8 {
                r: 0,
                g: 0xa0,
                b: 0,
            };
            data[2] = RGB8 {
                r: 0xa0,
                g: 0,
                b: 0,
            };
            data[3] = RGB8 {
                r: 0x40,
                g: 0x40,
                b: 0x40,
            };
            ws.write(data.iter().cloned()).unwrap();
            delay.delay_ms(1000 as u16);
            ws.write(empty.iter().cloned()).unwrap();
            delay.delay_ms(1000 as u16);
        }
/*        loop {
            // On for 1s, off for 1s.
            // https://doc.rust-lang.org/std/convert/enum.Infallible.html
            led.set_high().unwrap();
            xled.set_low().unwrap();
            delay.delay_ms(1000_u32);
            led.set_low().unwrap();
            xled.set_high().unwrap();
            delay.delay_ms(1000_u32);
        } */
    } else {
        panic!("failed to access peripherals");
    }
}
