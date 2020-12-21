#![no_std]
#![no_main]

use bl602_hal::{serial::*, pac, prelude::*, clock::Strict};

use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut parts = dp.GLB.split();
    // enable clock
    let clocks = Strict::new()
        .use_pll(40_000_000u32.Hz())
        .sys_clk(160_000_000u32.Hz())
        .uart_clk(40_000_000u32.Hz())
        .freeze(&mut parts.clk_cfg);
    let pin16 = parts.pin16.into_uart_sig0();
    let pin7 = parts.pin7.into_uart_sig7();
    let mux0 = parts.uart_mux0.into_uart0_tx();
    let mux7 = parts.uart_mux7.into_uart0_rx();
    let mut serial = Serial::uart0(
        dp.UART,
        Config::default().baudrate(20000.Bd()),
        ((pin16, mux0), (pin7, mux7)),
        clocks
    );
    loop {
        serial.try_write(b'R').ok();
        while !serial.try_flush().is_ok(){};
        serial.try_write(b'U').ok();
        while !serial.try_flush().is_ok(){};
        serial.try_write(b'S').ok();
        while !serial.try_flush().is_ok(){};
        serial.try_write(b'T').ok();
        while !serial.try_flush().is_ok(){};
    }
}
