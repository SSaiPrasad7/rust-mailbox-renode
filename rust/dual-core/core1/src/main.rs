#![no_std]
#![no_main]

use core::{ptr::read_volatile, ptr::write_volatile};
use panic_halt as _;
use riscv::{
    // asm::delay,
    register::{mcause, mie, mstatus, mtvec},
};
use riscv_rt::entry;
use udma_uart::Uart;

const SHARED_MEM_ADDR: usize = 0x1C08_0000;

const ITC_BASE_ADDR: usize = 0x1A10_9000;
const _SW_INT_ID: u8 = 3;
const EXT_INT_ID: u8 = 7;

const _ITC_MASK_OFFSET: usize = 0x00;
const ITC_MASK_SET_OFFSET: usize = 0x04;
const _ITC_MASK_CLR_OFFSET: usize = 0x08;

const _ITC_INT_OFFSET: usize = 0x0C;
const ITC_INT_SET_OFFSET: usize = 0x10;
const _ITC_INT_CLR_OFFSET: usize = 0x14;

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
            // udma_uart::sprintln!("Core 1: Hello from {}", "Interrupt Handler");
            
            let value = read_volatile((SHARED_MEM_ADDR) as *mut u32);
            udma_uart::sprintln!("Core 1: Reading value {} from shared memory.", value);
            write_volatile(SHARED_MEM_ADDR as *mut u32, value + 1);

            // Trigger external interrupt
            let interrupt_set_reg: *mut u32 = (ITC_BASE_ADDR + ITC_INT_SET_OFFSET) as *mut u32;
            write_volatile(interrupt_set_reg, 1 << EXT_INT_ID);

            // Enable global machine interrupts again
            mstatus::set_mie();
        }
        _ => {}
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
        // Enable machine software interrupts
        mie::set_msoft();

        // SET machine external interrupt in ITC PULP IRQ MASK register
        let mask_set_reg = (ITC_BASE_ADDR + ITC_MASK_SET_OFFSET) as *mut u32;
        write_volatile(mask_set_reg, 1 << EXT_INT_ID);

        // Enable global machine interrupts
        mstatus::set_mie();
    }

    loop {}
}
