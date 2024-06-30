#![no_std]
#![no_main]

use core::{arch::asm, ptr::read_volatile};
use panic_halt as _;
use riscv::register::{mcause, mie, mstatus, mtvec};
use riscv_rt::entry;
use udma_uart::Uart;

const SHARED_MEM_ADDR: usize = 0x1C08_0000;

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
            // udma_uart::sprintln!("Hello from {}", "Interrupt Handler");
            let value = read_volatile((SHARED_MEM_ADDR) as *mut u32);
            udma_uart::sprintln!("Core 1: Reading value {} from shared memory", value);
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
    udma_uart::sprintln!("Hello from {}", "Core 1");

    unsafe {
        // Register interrupt handler for machine software interrupt
        init_mtvec();

        // Enable machine software interrupts and global machine interrupts
        mie::set_msoft();
        mstatus::set_mie();
    }

    loop {
        unsafe { asm!("nop") }
    }
}
