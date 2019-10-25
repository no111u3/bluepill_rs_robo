//! Init function, RTFM

#![no_std]
#![no_main]

use panic_itm as _;

use stm32f1xx_hal as _;

use cortex_m::asm::wfi;
use rtfm::app;

use cortex_m_semihosting::hprintln;

#[app(device = stm32f1xx_hal::pac)]
const APP: () = {
    #[init]
    fn init() {
        hprintln!("init").unwrap();
    }

    #[idle]
    fn idle() -> ! {
        loop {
            // Waits for interrupt
            wfi();
        }
    }
};
