#![no_std]
#![no_main]

use core::{ptr::read_volatile, ptr::write_volatile};
use panic_halt as _;
use riscv::register::{mie, mstatus};
use riscv_rt::entry;
use udma_uart::Uart;

const SHARED_MEM_ADDR: usize = 0x1C08_0000;

const ITC_BASE_ADDR: usize = 0x1A10_9000;
const ITC_MASK_OFFSET: usize = 0x00;
const ITC_MASK_SET_OFFSET: usize = 0x04;
const ITC_INT_SET_OFFSET: usize = 0x10;

const SW_INT_ID: u8 = 3;

#[entry]
fn main() -> ! {
    let baud_rate = 115200;
    const UART_BASE_ADDR: usize = 0x1A10_2080;
    udma_uart::udma_uart_init(UART_BASE_ADDR, baud_rate);
    udma_uart::sprintln!("Hello from {}", "Core 0");

    unsafe {
        // Enable machine software interrupts and global machine interrupts
        mie::set_msoft();
        mstatus::set_mie();
    }

    let mask_set_reg = (ITC_BASE_ADDR + ITC_MASK_SET_OFFSET) as *mut u32;
    let interrupt_set_reg = (ITC_BASE_ADDR + ITC_INT_SET_OFFSET) as *mut u32;

    let mut write_value = 42;

    loop {
        unsafe {
            // Extract and check interrupt pending or not for machine software interrupt
            let int_reg_value = read_volatile((ITC_BASE_ADDR + ITC_MASK_OFFSET) as *mut u32);

            if (int_reg_value >> SW_INT_ID) & 1 == 0 {
                udma_uart::sprintln!("Core 0: Writing value {} to shared memory", write_value);
                write_volatile(SHARED_MEM_ADDR as *mut u32, write_value);
                write_value += 1;

                write_volatile(mask_set_reg, 1 << SW_INT_ID);
                write_volatile(interrupt_set_reg, 1 << SW_INT_ID);
            }
        }
    }
}
