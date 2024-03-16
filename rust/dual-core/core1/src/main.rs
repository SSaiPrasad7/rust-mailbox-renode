#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;
use core::ptr;
use core::arch::asm;

pub mod shared {
    #[link_section = ".shared_data"]
    #[no_mangle]
    pub static mut SHARED_VALUE: &'static str = " World";
}

#[entry]
fn main() -> ! {
    let clk_freq = 30000000;
    let baud_rate = 115200;

    let uart_clk_div = clk_freq / baud_rate;
    let uart_base_addr = 0x1a10_2100;
    let uart_tx_saddr_offset = 0x10;
    let uart_tx_size_offset = 0x14;
    let uart_tx_cfg_offset = 0x18;
    let uart_setup_offset = 0x24;
    let uart_config = (uart_clk_div << 16) | 0x317;

    for _ in 0..1000000 {
        unsafe { asm!("nop") }
    }

    unsafe {
        ptr::write_volatile((uart_base_addr + uart_setup_offset) as *mut u32, uart_config);
        ptr::write_volatile((uart_base_addr + uart_tx_saddr_offset) as *mut u32, shared::SHARED_VALUE.as_ptr() as u32);
        ptr::write_volatile((uart_base_addr + uart_tx_size_offset) as *mut u32, shared::SHARED_VALUE.len() as u32);
        ptr::write_volatile((uart_base_addr + uart_tx_cfg_offset) as *mut u32, 1 << 4);
        let value = ptr::read_volatile((uart_base_addr + uart_tx_saddr_offset) as *mut u32);
        while value != 0 {}
    }
    
    loop {}
}