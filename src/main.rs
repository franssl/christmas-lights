
#![no_main]
#![no_std]

// Halt on panic
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler

use cortex_m::{iprint};
use stm32f4xx_hal;

use crate::stm32f4xx_hal::{prelude::*, stm32};
use cortex_m_semihosting::{debug, hprintln};
use rtfm::cyccnt::{Instant, U32Ext};

const PERIOD: u32 = 48_000_000;

#[rtfm::app(device = stm32f4xx_hal::stm32, peripherals = true, monotonic = rtfm::cyccnt::CYCCNT)]
const APP: () = {

    struct Resources {
        led : stm32f4xx_hal::gpio::gpioa::PA5<stm32f4xx_hal::gpio::Output<stm32f4xx_hal::gpio::PushPull>>,
        is_on : bool,
        itm : cortex_m::peripheral::ITM

    }

    #[init(schedule = [blinky])]
    fn init(mut cx: init::Context) -> init::LateResources {
        //cp - core peripherals
        //let _core: cortex_m::Peripherals = cx.core;

        // Device specific peripherals
        let _device: stm32::Peripherals = cx.device;

        
        // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
        let gpioa = _device.GPIOA.split();
        let led = gpioa.pa5.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = _device.RCC.constrain();
        rcc.cfgr.sysclk(48.mhz()).freeze();
        

         // Initialize (enable) the monotonic timer (CYCCNT)
         cx.core.DCB.enable_trace();
         cx.core.DWT.enable_cycle_counter();
         let itm = cx.core.ITM;
        
        let now = cx.start;
        cx.schedule.blinky(now + PERIOD.cycles()).unwrap();
        init::LateResources { led, is_on: false, itm}
        
    }

    #[task(schedule = [blinky], resources = [led, itm])]
    fn blinky(cx: blinky::Context) {

        static mut IS_ON : bool = false;
        let led = cx.resources.led;
        
        if *IS_ON {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }
        *IS_ON=!*IS_ON;
        
        iprint!(&mut cx.resources.itm.stim[0], "{:?}", Instant::now());
        cx.schedule.blinky(cx.scheduled + PERIOD.cycles()).unwrap();
    }

    extern "C" {
        fn USART1();
    }
};
