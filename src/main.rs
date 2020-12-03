#![no_main]
#![no_std]
#[allow(unused_imports)]
//use embedded_hal::blocking::delay::{self, DelayMs};
use aux11::{entry, iprint, iprintln};
use heapless::{consts, Vec};
use core::str;
#[entry]
fn main() -> ! {
    loop {
    let (usart1, mono_timer, mut itm) = aux11::init();

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, consts::U64> = Vec::new();
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
                for (index, letter) in result.chars().enumerate() {
                    // if string does not start from the proper formatter, look again
                    if letter == '$' && index == 0 {
                        // make a statement from this character onward 
                        iprintln!(&mut itm.stim[0], "from extractor:\n{}",&result[index..]);
                    }                    
                }
                }
            // waiting via for loop
            let ms = 50_u16;        // this is not exactly an ms, but proportional
            delay(ms);
            // leds to be implemented further
            
            continue;
            }
    }
}


// complementray functions
fn delay(time:u16) {
    const K: u16 = 17_u16; // this value needs to be tweaked
    for _ in 0..(K * time) {
        aux11::nop()
        }
}

