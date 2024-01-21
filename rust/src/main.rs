#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;
use core::arch::asm;
use core::ptr;

#[entry]
fn main() -> ! {
        
    let clk_freq = 30000000;
    let baud_rate = 115200;

    let uart_clk_div = clk_freq / baud_rate;
    let uart_tx_saddr = 0x10;
    let uart_tx_size = 0x14;
    let uart_tx_cfg = 0x18;
    let uart_setup_offset = 0x24;
    let uart_base_addr = 0x1a10_2100;
    let uart_config = (uart_clk_div << 16) | 0x317;
    let s = "\r\nHello World\r\n";

    unsafe {
        ptr::write_volatile((uart_base_addr + uart_setup_offset) as *mut u32, uart_config);
        ptr::write_volatile((uart_base_addr + uart_tx_saddr) as *mut u32, s.as_ptr() as u32);
        ptr::write_volatile((uart_base_addr + uart_tx_size) as *mut u32, s.len() as u32);
        ptr::write_volatile((uart_base_addr + uart_tx_cfg) as *mut u32, 1 << 4);
        let value = ptr::read_volatile((uart_base_addr + uart_tx_saddr) as *mut u32);
        while value != 0 {}
    }
    
    // Busy-wait a moment for extra safety
    for _ in 0..10_000 {
        unsafe {
            asm!("nop");
        }
    }

    loop { continue; }
}