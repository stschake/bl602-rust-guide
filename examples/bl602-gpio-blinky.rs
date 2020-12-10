#![no_std]
#![no_main]

use bl602_hal::{pac, prelude::*};

use panic_halt as _;
use bl602_hal::delay::McycleDelay;
#[riscv_rt::entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();
    bl602_hal::clock::glb_set_system_clk(&mut dp);
    let parts = dp.GLB.split();
    let mut gpio5 = parts.pin5.into_pull_down_output();
    gpio5.try_set_high().unwrap();
    
    // We should be running at 160MHz, but the glb_set_system_clk code isn't working correctly
    // Set the sysclock reg to the freq we are running at
    dp.HBN.hbn_rsv2.write(|w| unsafe {w.bits(32_000_000)});
    let current_freq = dp.HBN.hbn_rsv2.read().bits();

    let mut d = bl602_hal::delay::McycleDelay::new(current_freq);
    loop {
        d.try_delay_ms(1000);
        gpio5.try_toggle().unwrap();
    }
}
