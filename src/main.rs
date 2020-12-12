#![no_main]
#![no_std]
#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};
use heapless::{consts, Vec};
use core::str;
use stm32f3xx_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
#[entry]
fn main() -> ! {
    let (usart1, mono_timer, mut itm, mut delay, gpioe, rcc) = aux11::init();

    // data container
    let mut buffer: Vec<u8, consts::U128> = Vec::new();
    // enabling gpioe peripheral
    rcc.ahbenr.modify(|_, w| w.iopeen().set_bit());
    // configuring pins as output pins
    gpioe.moder.modify(|_, w| {
        w.moder8().output();
        w.moder9().output()
    });
    loop {
        buffer.clear();
        //reading data via uart

        loop {
            while usart1.isr.read().rxne().bit_is_clear() {}
            let byte = usart1.rdr.read().rdr().bits();
            

            if buffer.push(byte as u8).is_err() {		//buffer full, break
                break;
            }
        }
            
        unsafe {let result = str::from_utf8_unchecked(& buffer);
            // getting data in GNMRC and GNGGA format
            let mut start = 0;
            let mut end = 0;
            for (index, letter) in result.chars().enumerate() {
                if letter == '$' {
                    start = index;}
                    else if letter == '\n' {
                        // selecting GNMRC or GGA data for further parsing
                        if &result[start..(start + 6)] == "$GNRMC" || &result[start..(start + 6)] == "$GNGGA" {
                            iprintln!(&mut itm.stim[0],"Data: {}", &result[start..index]);
                        }
                    }    
                }
            }                   
        let ms = 500_u16;			// delay time
        // acuator operates based on the result of location
        // implementation ommitted
        // signal to the actuator if location is valid
        let mut location = true;
        if location { 		// valid location, actuator is on
        gpioe.odr.write(|w| {			// LED as actuator
            w.odr9().set_bit()
        });}
        else {			// invalid location, actuator off
			gpioe.bsrr.write(|w| w.br9().set_bit());
			}
        delay.delay_ms(ms);
        }    
}