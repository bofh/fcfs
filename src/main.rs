#![no_std]
#![no_main]
#![feature(core_intrinsics)]

use core::intrinsics;
use core::panic::PanicInfo;
use cortex_m_rt::entry;
//use hal::stm32f30x::{self, interrupt, Interrupt};
use hal::gpio::{MediumSpeed, PullUp};
use hal::timer;
use hal::time::KiloHertz;//{Bps, Hertz, KiloHertz};
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let device = hal::stm32f30x::Peripherals::take().unwrap();
    let mut rcc = device.RCC.constrain();
    let gpioa = device.GPIOA.split(&mut rcc.ahb);
    let mut flash = device.FLASH.constrain();
    let clocks = rcc.cfgr
                    .sysclk(64.mhz())
                    .pclk1(32.mhz())
                    .pclk2(32.mhz())
                    .freeze(&mut flash.acr);

    let (ch1, ch2, ch3, ch4, mut timer2) =
        timer::tim2::Timer::new(device.TIM2,
                                KiloHertz(32),
                                clocks,
                                &mut rcc.apb1).take_all();
    let mut m_rear_right = gpioa.pa0.pull_type(PullUp).to_pwm(ch1, MediumSpeed);
    let mut m2_front_right =
        gpioa.pa1.pull_type(PullUp).to_pwm(ch2, MediumSpeed);
    let mut m3_rear_left = gpioa.pa2.pull_type(PullUp).to_pwm(ch3, MediumSpeed);
    let mut m4_front_left =
        gpioa.pa3.pull_type(PullUp).to_pwm(ch4, MediumSpeed);
    m_rear_right.enable();
    m2_front_right.enable();
    m3_rear_left.enable();
    m4_front_left.enable();
    timer2.enable();

    loop {
    };
}

#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}

