#![no_std]
#![no_main]
#![feature(core_intrinsics)]

use core::intrinsics;
use core::panic::PanicInfo;
use cortex_m_rt::entry;
use hal::stm32f30x::{self, interrupt, Interrupt};

#[entry]
fn main() -> ! {
    loop {
    };
}

#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}

