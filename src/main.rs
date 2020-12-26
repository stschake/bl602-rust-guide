#![no_std]
#![no_main]

use core::convert::Infallible;
use bl602_hal::{serial::*, pac, prelude::*, delay::*};
use ufmt::{uWrite, uwriteln};
use panic_halt as _;

struct Printer<T>(pub T);
impl<T> uWrite for Printer<T>
where
    T: embedded_hal::serial::Write<u8>
{
    type Error = Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        let bytes = s.as_bytes();
        for b in bytes.iter() {
            self.0.try_write(*b).ok();
            while !self.0.try_flush().is_ok() {};
        }
        Ok(())
    }
}

#[riscv_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    // Set fclk as clock source for UART while we have access to peripheral registers
    dp.HBN.hbn_glb.modify(|_,w| { w
        .hbn_uart_clk_sel().clear_bit()
    });
    let mut parts = dp.GLB.split();
    // enable clock
    let clocks = bl602_hal::clock::Strict::new()
        .use_pll(40_000_000u32.Hz())
        .sys_clk(160_000_000u32.Hz())
        .freeze(&mut parts.clk_cfg)
        .freeze();

    let pin16 = parts.pin16.into_uart_sig0();
    let pin7 = parts.pin7.into_uart_sig7();
    let mux0 = parts.uart_mux0.into_uart0_tx();
    let mux7 = parts.uart_mux7.into_uart0_rx();
    let serial = Serial::uart0(
        dp.UART,
        Config::default().baudrate(2_000_000.Bd()),
        ((pin16, mux0), (pin7, mux7)),
        clocks
    );

    let sys_clk_freq = bl602_hal::clock::system_core_clock_get();
    let mut delay = McycleDelay::new(sys_clk_freq);
    let mut printer = Printer(serial);

    // configure serial flash
    dp.SF_CTRL.sf_ctrl_0.modify(|_,w| { w
        .sf_if_read_dly_en().set_bit()
        .sf_clk_sf_rx_inv_sel().set_bit()
    });

    loop {
        uwriteln!(&mut printer, "cycle {}\r", McycleDelay::get_time());
        delay.try_delay_ms(1000).ok();
    }
}
