#![no_main]
#![no_std]
#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};
use heapless::{consts, Vec};
use core::str;
//use stm32f3xx_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
//use embedded_hal::digital::v2::OutputPin;
#[entry]
fn main() -> ! {
    loop {
    let (usart1, mono_timer, mut itm, _, _, _) = aux11::init();

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, consts::U64> = Vec::new();
    // enabling gpioe peripheral
    //rcc.ahbenr.modify(|_, w| w.iopeen().set_bit());
    //// configuring pins as output pins
    //gpioe.moder.modify(|_, w| {
    //    w.moder8().output();
    //    w.moder9().output()
    //});
    loop {
        buffer.clear();

        loop {          // this loop reads the data via uart
            while usart1.isr.read().rxne().bit_is_clear() {}
            let byte = usart1.rdr.read().rdr().bits();
            

            if buffer.push(byte as u8).is_err() {
                iprintln!(&mut itm.stim[0], "buffer full");
                break;
            }
        //        // buffer full
        //        for byte in b"error: buffer full\n\r" {
        //            while usart1.isr.read().txe().bit_is_clear() {}
        //            unsafe {
        //            usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
        //            }
        //        }
        //        break;
        //}
            // converting data to string
            //let data = buffer.len();
            // Carriage return
            //if byte == 13 {
                // Respond
            //let result = str::from_utf8(&buffer).unwrap();
            //iprintln!(&mut itm.stim[0], "This is the resultant string {:?}", result);
            //iprintln!(&mut itm.stim[0], "buffer length: {}", buffer.len());
            //    for byte in buffer.iter() {
            //iprintln!(&mut itm.stim[0], "data: {:#?}", byte);
            //} next


            
            //iprintln!(&mut itm.stim[0], "buffer length: {}", buffer.len());
            //    for byte in buffer.iter() {
            //iprintln!(&mut itm.stim[0], "data: {:#?}", byte); 

            //break;
            }

            unsafe {let result = str::from_utf8_unchecked(& buffer);
                iprintln!(&mut itm.stim[0], "raw string\n {}", result);
                // try to apply trim to drop garbade from the data
                let mut nmea_start = 0;
                for (index, letter) in result.chars().enumerate() {
                    // if string does not start from the proper formatter, look again
                    if letter == '$' {
                        // make a statement from this character onward
                        let nmea_start = index;}
                    if letter == '\n' {
                        iprintln!(&mut itm.stim[0], "{}",&result[nmea_start..index]);}
                    }                    
                }
            // waiting via for loop
            let ms = 50_u16;        // this is not exactly an ms, but proportional
            //delay.delay_ms(ms);
            delay_time(ms);
            // leds to be implemented further
            // Turn on all the LEDs in the compass
            //gpioe.odr.write(|w| {
            //    w.odr8().set_bit();
            //    w.odr9().set_bit()
            //});
            ////delay.delay_ms(ms);
            //delay_time(ms);
            //// turn off leds
            //gpioe.bsrr.write(|w| w.br8().set_bit());
            //gpioe.bsrr.write(|w| w.br9().set_bit());
            // work to be done to continue loop,
            continue;
            }
    }
}


// complementray functions
fn delay_time(time:u16) {
    const K: u16 = 300_u16; // this value needs to be tweaked
    for _ in 0..(K * time) {
        aux11::nop()
        }
}

