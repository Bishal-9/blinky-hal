#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use stm32f3_discovery::stm32f3xx_hal::{
    delay::Delay,
    pac,
    prelude::*
};

#[entry]
fn main() -> ! {

    // Get access to the core peripherals using cortex-m crate
    let core_peripheral = cortex_m::Peripherals::take().expect("Core Peripheral Error occurred.");

    // Get access ot device specific peripherals using the peripheral access crate
    let device_peripheral = pac::Peripherals::take().expect("Device Specific Peripheral Error occurred.");

    // Take ownership of the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = device_peripheral.FLASH.constrain();
    let mut rcc = device_peripheral.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and retrieve the clock frequencies
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Get the delay provider
    let mut delay = Delay::new(core_peripheral.SYST, clocks);

    // Acquire the GPIOE peripheral
    let mut gpioe = device_peripheral.GPIOE.split(&mut rcc.ahb);

    // Configure the LEDs as outputs
    let mut led_8 = gpioe.pe8.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led_9 = gpioe.pe9.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led_10 = gpioe.pe10.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led_11 = gpioe.pe11.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led_12 = gpioe.pe12.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led_13 = gpioe.pe13.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led_14 = gpioe.pe14.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led_15 = gpioe.pe15.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    loop {
        
        led_8.set_high().expect("Setting LED 8 pin high error");
        delay.delay_ms(50_u16);

        led_8.set_low().expect("Setting LED 8 pin low error");
        delay.delay_ms(50_u16);

        led_9.set_high().expect("Setting LED 9 pin high error");
        delay.delay_ms(50_u16);

        led_9.set_low().expect("Setting LED 9 pin low error");
        delay.delay_ms(50_u16);

        led_10.set_high().expect("Setting LED 10 pin high error");
        delay.delay_ms(50_u16);

        led_10.set_low().expect("Setting LED 10 pin low error");
        delay.delay_ms(50_u16);

        led_11.set_high().expect("Setting LED 11 pin high error");
        delay.delay_ms(50_u16);

        led_11.set_low().expect("Setting LED 11 pin low error");
        delay.delay_ms(50_u16);

        led_12.set_high().expect("Setting LED 12 pin high error");
        delay.delay_ms(50_u16);

        led_12.set_low().expect("Setting LED 12 pin low error");
        delay.delay_ms(50_u16);

        led_13.set_high().expect("Setting LED 13 pin high error");
        delay.delay_ms(50_u16);

        led_13.set_low().expect("Setting LED 13 pin low error");
        delay.delay_ms(50_u16);

        led_14.set_high().expect("Setting LED 14 pin high error");
        delay.delay_ms(50_u16);

        led_14.set_low().expect("Setting LED 14 pin low error");
        delay.delay_ms(50_u16);

        led_15.set_high().expect("Setting LED 15 pin high error");
        delay.delay_ms(50_u16);

        led_15.set_low().expect("Setting LED 15 pin low error");
        delay.delay_ms(50_u16);
    }
}
