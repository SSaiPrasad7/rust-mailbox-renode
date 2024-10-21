#![no_std]
#![no_main]
#![allow(unused)]
#![allow(static_mut_refs)]

use core::ptr::write_volatile;
use panic_halt as _;
use riscv::{
    // asm::delay,
    register::{mcause, mie, mstatus, mtvec},
};
use riscv_rt::entry;
use udma_uart::Uart;

const ITC_BASE_ADDR: usize = 0x1A10_9000;
const SW_INTERRUPT_ID_16: u8 = 16;
const SW_INTERRUPT_ID_17: u8 = 17;

const ITC_MASK_OFFSET: usize = 0x00;
const ITC_MASK_SET_OFFSET: usize = 0x04;
const ITC_MASK_CLR_OFFSET: usize = 0x08;

const ITC_INT_OFFSET: usize = 0x0C;
const ITC_INT_SET_OFFSET: usize = 0x10;
const ITC_INT_CLR_OFFSET: usize = 0x14;
const MAILBOX_BUFFER_SIZE: usize = 2048; // 2KB

#[link_section = ".shared"]
static mut MAILBOX: [u32; MAILBOX_BUFFER_SIZE] = [0; MAILBOX_BUFFER_SIZE];

fn write_to_mailbox(data: u32, index: usize) {
    unsafe {
        MAILBOX[index] = data;
    }
}

fn read_from_mailbox(index: usize) -> u32 {
    unsafe { MAILBOX[index] }
}

unsafe fn init_mtvec() {
    extern "C" {
        fn _mfast_soft_int_handler();
    }
    mtvec::write(_mfast_soft_int_handler as usize, mtvec::TrapMode::Direct);
}

#[no_mangle]
unsafe extern "C" fn _mfast_soft_int_handler() {
    match mcause::read().cause() {
        mcause::Trap::Interrupt(mcause::Interrupt::MachineFast16) => {
            // udma_uart::sprintln!("Core 1: Hello from {}", "Interrupt Handler");
            // let value = read_from_mailbox(1);
            // udma_uart::sprintln!("Core 1: Reading value {} from shared memory.", value);
            // write_to_mailbox(value + 1, 1);
            udma_uart::sprintln!("Core 1: Reading the mailbox:");
            udma_uart::sprintln!("{:?}", MAILBOX);

            // Clear machine fast software interrupt number 16
            let interrupt_clr_reg = (ITC_BASE_ADDR + ITC_INT_CLR_OFFSET) as *mut u32;
            write_volatile(interrupt_clr_reg, 1 << SW_INTERRUPT_ID_16);

            // // Trigger machine fast software interrupt number 17
            // let interrupt_set_reg: *mut u32 = (ITC_BASE_ADDR +
            // ITC_INT_SET_OFFSET) as *mut u32;
            // write_volatile(interrupt_set_reg, 1 << SW_INTERRUPT_ID_17);

            // // Enable global machine interrupts again
            // mstatus::set_mie();
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
        // Register machine fast software interrupt handler
        init_mtvec();
        // Enable machine fast software interrupt
        mie::set_mfast_16();

        // SET machine fast software interrupt number 17 in ITC PULP IRQ MASK register
        let mask_set_reg: *mut u32 = (ITC_BASE_ADDR + ITC_MASK_SET_OFFSET) as *mut u32;
        write_volatile(mask_set_reg, 1 << SW_INTERRUPT_ID_17);

        // Enable global machine interrupts
        mstatus::set_mie();
    }

    loop {}
}
