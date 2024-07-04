#![no_std]
#![no_main]

use core::{ptr::read_volatile, ptr::write_volatile};
use panic_halt as _;
use riscv::{
    asm::delay,
    register::{mcause, mie, mstatus, mtvec},
};
use riscv_rt::entry;
use udma_uart::Uart;

const ITC_BASE_ADDR: usize = 0x1A10_9000;
const SW_INT_ID: u8 = 3;

const _ITC_MASK_OFFSET: usize = 0x00;
const ITC_MASK_SET_OFFSET: usize = 0x04;
const _ITC_MASK_CLR_OFFSET: usize = 0x08;

const ITC_INT_OFFSET: usize = 0x0C;
const ITC_INT_SET_OFFSET: usize = 0x10;
const ITC_INT_CLR_OFFSET: usize = 0x14;

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

            // Clear machine software interrupt in ITC PULP IRQ INTERRUPT register
            let interrupt_clr_reg: *mut u32 = (ITC_BASE_ADDR + ITC_INT_CLR_OFFSET) as *mut u32;
            write_volatile(interrupt_clr_reg, 1 << SW_INT_ID);
        }
        _ => {}
    }
}

#[entry]
fn main() -> ! {
    let baud_rate = 115200;
    const UART_BASE_ADDR: usize = 0x1A10_2100;
    udma_uart::udma_uart_init(UART_BASE_ADDR, baud_rate);
    // udma_uart::sprintln!("Hello from {}", "main");

    unsafe {
        init_mtvec();
        // Enable machine software interrupts
        mie::set_msoft();
        // SET machine software interrupt in ITC PULP IRQ MASK register
        let mask_set_reg = (ITC_BASE_ADDR + ITC_MASK_SET_OFFSET) as *mut u32;
        write_volatile(mask_set_reg, 1 << SW_INT_ID);
    }

    loop {
        unsafe {
            // Extract and check interrupt pending or not for machine software interrupt
            let int_reg_value = read_volatile((ITC_BASE_ADDR + ITC_INT_OFFSET) as *mut u32);
            if (int_reg_value >> SW_INT_ID) & 1 == 0 {
                delay(37_000_000);

                // Set machine software interrupt in ITC PULP IRQ INTERRUPT register
                let interrupt_set_reg: *mut u32 = (ITC_BASE_ADDR + ITC_INT_SET_OFFSET) as *mut u32;
                write_volatile(interrupt_set_reg, 1 << SW_INT_ID);

                // Enable global machine interrupts
                mstatus::set_mie();
            }
        }
    }
}
