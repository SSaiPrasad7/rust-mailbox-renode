#![no_std]
#![no_main]

use core::{arch::asm, ptr::write_volatile};
use panic_halt as _;
use riscv::register::{mcause, mie, mstatus, mtvec};
use riscv_rt::entry;
use udma_uart::Uart;

const ITC_BASE_ADDR: usize = 0x1A10_9000;
const ITC_MASK_SET_OFFSET: usize = 0x04;
const ITC_INT_SET_OFFSET: usize = 0x10;

unsafe fn init_mtvec() {
    extern "C" {
        fn _msoft_int_handler();
    }
    mtvec::write(_msoft_int_handler as usize, mtvec::TrapMode::Direct);
}

#[no_mangle]
unsafe extern "C" fn _msoft_int_handler() {
    match mcause::read().cause() {
        mcause::Trap::Interrupt(mcause::Interrupt::MachineSoft) => {
            udma_uart::sprintln!("Hello from {}", "Interrupt Handler");
        }
        _ => {
            udma_uart::sprintln!("Not Machine Soft Interrupt");
        }
    }
}

#[entry]
fn main() -> ! {
    let baud_rate = 115200;
    const UART_BASE_ADDR: usize = 0x1A10_2100;
    udma_uart::udma_uart_init(UART_BASE_ADDR, baud_rate);
    udma_uart::sprintln!("Hello {}", "from main function");
    unsafe {
        init_mtvec();
        // Enable machine software interrupts and global machine interrupts
        mie::set_msoft();
        mstatus::set_mie();

        const SW_INT_ID: u8 = 3;
        let mask_set_reg = (ITC_BASE_ADDR + ITC_MASK_SET_OFFSET) as *mut u32;
        write_volatile(mask_set_reg, 1 << SW_INT_ID);

        let interrupt_set_reg = (ITC_BASE_ADDR + ITC_INT_SET_OFFSET) as *mut u32;
        write_volatile(interrupt_set_reg, 1 << SW_INT_ID);
    }

    loop {
        unsafe { asm!("nop") }
    }
}
