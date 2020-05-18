#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux6::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let mut itm = aux6::init();

    iprintln!(&mut itm.stim[0], "Hello, worldddd from the terminal who wooowh ooooo!");

    loop {}
}
