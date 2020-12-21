#![no_std]
#![no_main]

use bl602_hal::{pac, prelude::*, clock::*};
use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();
    // enable clock
    let clock =  bl602_hal::clock::Clocks::new()
        .use_pll(40_000_000u32.Hz())
        .sys_clk(160_000_000u32.Hz())
        .freeze()
        .freeze();
    let sys_clk_freq = bl602_hal::clock::system_core_clock_get();
    let parts = dp.GLB.split();
    let mut gpio5 = parts.pin5.into_pull_down_output();
    gpio5.try_set_high().unwrap();
    
    // Create a blocking delay function based on the current core clock
    let mut d = bl602_hal::delay::McycleDelay::new(sys_clk_freq);

    loop {
        d.try_delay_ms(1000).unwrap();
        gpio5.try_toggle().unwrap();
    }
}
