#![no_std]
#![no_main]

use bl602_hal::{pac, prelude::*, clock::*};
use panic_halt as _;
#[riscv_rt::entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();
    // enable clock
    bl602_hal::clock::glb_set_system_clk(
        &mut dp,
        GLB_PLL_XTAL_Type::XTAL_40M,
        sys_clk::PLL160M
    );
    let sys_clk_freq = bl602_hal::clock::SystemCoreClockGet(&mut dp);
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
