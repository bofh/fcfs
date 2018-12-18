#![no_std]
#![no_main]
#![feature(core_intrinsics)]

use core::intrinsics;
use core::panic::PanicInfo;
use cortex_m_rt::entry;
use hal::stm32f30x::{self, interrupt};
use hal::gpio::PullUp;

use hal::time::Bps;
use hal::prelude::*;
use core::fmt::{self, Write};

extern crate cortex_m;

use cortex_m::peripheral::Peripherals;
use nb;

static mut L: Option<Logger<hal::serial::Tx<hal::stm32f30x::USART1>>> = None;

#[entry]
fn main() -> ! {
    let device = hal::stm32f30x::Peripherals::take().unwrap();
    let mut core = Peripherals::take().unwrap();

    unsafe { cortex_m::interrupt::enable() };

    // Enable the EXTI13 interrupt
    core.NVIC.enable(
        stm32f30x::Interrupt::EXTI15_10,
    );
    // Connect GPIOC13 to EXTI13
    device.SYSCFG.exticr4.modify(|_, w| unsafe {
        w.exti13().bits(0b010)
    });
    device.RCC.apb2enr.write(|w| w.syscfgen().enabled());
    // Enable interrupt on rise
    device.EXTI.imr1.modify(|_, w| w.mr13().set_bit());
    device.EXTI.emr1.modify(|_, w| w.mr13().set_bit());
    device.EXTI.rtsr1.modify(|_, w| w.tr13().set_bit());

    // Construct logger
    let mut rcc = device.RCC.constrain();
    let mut flash = device.FLASH.constrain();
    let clocks = rcc
        .cfgr
        .sysclk(64.mhz())
        .pclk1(32.mhz())
        .pclk2(32.mhz())
        .freeze(&mut flash.acr);

    let gpioa = device.GPIOA.split(&mut rcc.ahb);
    let serial = device
        .USART1
        .serial((gpioa.pa9, gpioa.pa10), Bps(115200), clocks);
    let (tx, _) = serial.split();

    // Use PC13 as input
    let gpioc = device.GPIOC.split(&mut rcc.ahb);
    let pc13 = gpioc.pc13.pull_type(PullUp).input();

    unsafe {
        L = Some(Logger { tx });
    };

    let l = unsafe { extract(&mut L) };
    write!(l, "logger ok\r\n").unwrap();
    if pc13.is_low() {
        write!(l, "low\r\n").unwrap();
    }

    loop {
        cortex_m::asm::nop(); // avoid rust-lang/rust#28728
    };
}

interrupt!(EXTI15_10, exti13);
fn exti13() {
    let l = unsafe { extract(&mut L) };
    write!(l, "i").unwrap();
}

#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}

struct Logger<W: ehal::serial::Write<u8>> {
    tx: W,
}
impl<W: ehal::serial::Write<u8>> fmt::Write for Logger<W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            match self.write_char(c) {
                Ok(_) => {}
                Err(_) => {}
            }
        }
        match self.tx.flush() {
            Ok(_) => {}
            Err(_) => {}
        };

        Ok(())
    }

    fn write_char(&mut self, s: char) -> fmt::Result {
        match nb::block!(self.tx.write(s as u8)) {
            Ok(_) => {}
            Err(_) => {}
        }
        Ok(())
    }
}

unsafe fn extract<T>(opt: &'static mut Option<T>) -> &'static mut T {
    match opt {
        Some(ref mut x) => &mut *x,
        None => panic!("extract"),
    }
}