#![no_std]
#![no_main]

use bl602_hal::{pac, clock::*};
use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();
    // enable clock
    bl602_hal::clock::glb_set_system_clk(
        GlbPllXtalType::Xtal40m,
        SysClk::Pll160m
    );

    // Set fclk as clock source for UART
    dp.HBN.hbn_glb.modify(|r,w| unsafe { w
        .hbn_uart_clk_sel().clear_bit()
    });

    // calculate baudrate
    let target_baudrate = 19200;
    let sysclk = bl602_hal::clock::system_core_clock_get();
    let uart_clk_div = 4; // reset
    let baudrate_divisor = (sysclk / (uart_clk_div + 1) / target_baudrate) as u16;
    //let baudrate_divisor = 200;  // 160M / 4 / 200 = 20K baud
    dp.GLB.clk_cfg2.write(|w| unsafe { w
        .uart_clk_div().bits(uart_clk_div as u8)
        .uart_clk_en().set_bit()
    });

    dp.UART.uart_bit_prd.write(|w| unsafe { w
        .cr_urx_bit_prd().bits(baudrate_divisor - 1)
        .cr_utx_bit_prd().bits(baudrate_divisor - 1)
    });
    // no bit inverse
    dp.UART.data_config.write(|w| w
        .cr_uart_bit_inv().clear_bit()
    );
    // 8N1
    /* 4->5b 5->6b 6->7b 7->8b */
    let data_bits_cfg = 7; // 8 bits
    /* 0->0.5b 1->1b 2->1.5b 3->2b */
    let stop_bits_cfg = 1; // todo: check this parameter
    dp.UART.utx_config.write(|w| unsafe { w
        .cr_utx_prt_en().clear_bit() // parity: none
        .cr_utx_bit_cnt_d().bits(data_bits_cfg)
        .cr_utx_bit_cnt_p().bits(stop_bits_cfg) 
        .cr_utx_frm_en().set_bit() // freerun on
        // freerun off
        .cr_utx_cts_en().clear_bit() // no CTS
        .cr_utx_en().set_bit() // enable TX
    });
    dp.UART.urx_config.write(|w| unsafe { w
        .cr_urx_prt_en().clear_bit() // parity: none
        .cr_urx_bit_cnt_d().bits(data_bits_cfg)
        .cr_urx_deg_en().clear_bit() // no rx input de-glitch
        .cr_urx_rts_sw_mode().clear_bit() // no RTS
        .cr_urx_en().set_bit() // enable RX
    });
    // set gpio configuration
    // tx pin
    dp.GLB.gpio_cfgctl8.modify(|_, w| unsafe { w
        .reg_gpio_16_func_sel().bits(7) // GPIO_FUN_UART
        .reg_gpio_16_ie().set_bit() // input
        .reg_gpio_16_pu().set_bit() // pull up enable
        .reg_gpio_16_pd().clear_bit()
        .reg_gpio_16_drv().bits(0) // disabled
        .reg_gpio_16_smt().clear_bit()
    });
    // rx pin
    dp.GLB.gpio_cfgctl3.modify(|_, w| unsafe { w
        .reg_gpio_7_func_sel().bits(7) // GPIO_FUN_UART
        .reg_gpio_7_ie().set_bit() // input
        .reg_gpio_7_pu().set_bit() // pull up enable
        .reg_gpio_7_pd().clear_bit()
        .reg_gpio_7_drv().bits(0) // disabled
        .reg_gpio_7_smt().clear_bit()
    });
    dp.GLB.uart_sig_sel_0.write(|w| unsafe { w
        .uart_sig_0_sel().bits(2) // tx -> GLB_UART_SIG_FUN_UART0_TXD
        .uart_sig_7_sel().bits(3) // rx -> GLB_UART_SIG_FUN_UART0_RXD
    });
    loop {
        // write data
        while dp.UART.uart_fifo_config_1.read().tx_fifo_cnt().bits() < 1 {}
        dp.UART.uart_fifo_wdata.write(|w| unsafe {
            w.bits(b'R' as u32)
        });
        while dp.UART.uart_fifo_config_1.read().tx_fifo_cnt().bits() < 1 {}
        dp.UART.uart_fifo_wdata.write(|w| unsafe {
            w.bits(b'U' as u32)
        });
        while dp.UART.uart_fifo_config_1.read().tx_fifo_cnt().bits() < 1 {}
        dp.UART.uart_fifo_wdata.write(|w| unsafe {
            w.bits(b'S' as u32)
        });
        while dp.UART.uart_fifo_config_1.read().tx_fifo_cnt().bits() < 1 {}
        dp.UART.uart_fifo_wdata.write(|w| unsafe {
            w.bits(b'T' as u32)
        });
    }
}
