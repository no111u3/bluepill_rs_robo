#![no_std]
#![no_main]

use panic_itm as _;

use stm32f1xx_hal as _;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {}
}
