#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;
use core::arch::asm;
use core::ptr;

pub mod shared {
    #[link_section = ".shared_data"]
    #[no_mangle]
    pub static mut SHARED_VALUE: usize = 100;
}

// fn num_to_str(mut n: usize, buffer: &mut [u8]) -> usize {
//     let mut i = buffer.len();

//     // Convert the number to a string in reverse order
//     loop {
//         i -= 1;
//         buffer[i] = (n % 10) as u8 + b'0';
//         n /= 10;

//         if n == 0 || i == 0 {
//             break;
//         }
//     }

//     buffer.len() - i
// }

// unsafe fn concat_str_and_usize(s: &str, n: usize) -> &[u8] {
//     let mut num_str = [0u8; 20];
//     let len = num_to_str(n, &mut num_str);
//     let total_len = s.len() + len;

//     let concat_str = core::slice::from_raw_parts(
//         s.as_ptr(),
//         total_len.min(s.len()),
//     );

//     concat_str
// }

// unsafe fn uart_write(uart_base_addr: usize, buffer: &str)
// {
//     let uart_tx_saddr_offset = 0x10;
//     let uart_tx_size_offset = 0x14;
//     let uart_tx_cfg_offset = 0x18;

//     ptr::write_volatile((uart_base_addr + uart_tx_saddr_offset) as *mut u32, buffer.as_ptr() as u32);
//     ptr::write_volatile((uart_base_addr + uart_tx_size_offset) as *mut u32, buffer.len() as u32);
//     ptr::write_volatile((uart_base_addr + uart_tx_cfg_offset) as *mut u32, 1 << 4);
//     let value = ptr::read_volatile((uart_base_addr + uart_tx_saddr_offset) as *mut u32);
//     while value != 0 {}
// }

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
    let s = "\r\nHello World0\r\n";

    unsafe {
        ptr::write_volatile((uart_base_addr + uart_setup_offset) as *mut u32, uart_config);
        ptr::write_volatile((uart_base_addr + uart_tx_saddr_offset) as *mut u32, s.as_ptr() as u32);
        ptr::write_volatile((uart_base_addr + uart_tx_size_offset) as *mut u32, s.len() as u32);
        ptr::write_volatile((uart_base_addr + uart_tx_cfg_offset) as *mut u32, 1 << 4);
        let value = ptr::read_volatile((uart_base_addr + uart_tx_saddr_offset) as *mut u32);
        while value != 0 {}
    }
    
    // Busy-wait a moment for extra safety
    for _ in 0..10_000 {
        unsafe {
            asm!("nop");
        }
    }
    // unsafe {
        // let shared_val = ptr::read_volatile(&shared::SHARED_VALUE);
        // ptr::write_volatile(&mut shared::SHARED_VALUE, shared_val + 1);
        // let buffer = concat_str_and_usize(s, shared_val);
    // }

    loop {continue;}
}