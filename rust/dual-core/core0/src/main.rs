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
const SW_INTERRUPT_ID_16: u8 = 16;
const SW_INTERRUPT_ID_17: u8 = 17;

const _ITC_MASK_OFFSET: usize = 0x00;
const ITC_MASK_SET_OFFSET: usize = 0x04;
const _ITC_MASK_CLR_OFFSET: usize = 0x08;

const _ITC_INT_OFFSET: usize = 0x0C;
const ITC_INT_SET_OFFSET: usize = 0x10;
const ITC_INT_CLR_OFFSET: usize = 0x14;

unsafe fn init_mtvec() {
    extern "C" {
        fn _mfast_soft_int_handler();
    }
    mtvec::write(_mfast_soft_int_handler as usize, mtvec::TrapMode::Direct);
}

#[no_mangle]
unsafe extern "C" fn _mfast_soft_int_handler() {
    match mcause::read().cause() {
        mcause::Trap::Interrupt(mcause::Interrupt::MachineFast17) => {
            // udma_uart::sprintln!("Core 0: Hello from {}", "Interrupt Handler");

            let value = read_volatile((SHARED_MEM_ADDR) as *mut u32);
            udma_uart::sprintln!("Core 0: Reading value {} from shared memory.", value);
            write_volatile(SHARED_MEM_ADDR as *mut u32, value + 1);

            // Clear machine fast software interrupt number 17
            let interrupt_clr_reg = (ITC_BASE_ADDR + ITC_INT_CLR_OFFSET) as *mut u32;
            write_volatile(interrupt_clr_reg, 1 << SW_INTERRUPT_ID_17);

            // Trigger machine fast software interrupt number 16
            let interrupt_set_reg: *mut u32 = (ITC_BASE_ADDR + ITC_INT_SET_OFFSET) as *mut u32;
            write_volatile(interrupt_set_reg, 1 << SW_INTERRUPT_ID_16);

            // Enable global machine interrupts again
            mstatus::set_mie();
        }
        _ => {}
    }
}

#[entry]
fn main() -> ! {
    let baud_rate = 115200;
    const UART_BASE_ADDR: usize = 0x1A10_2080;
    udma_uart::udma_uart_init(UART_BASE_ADDR, baud_rate);
    udma_uart::sprintln!("Hello from {}", "Core 0");

    unsafe {
        // Register machine fast software interrupt handler
        init_mtvec();
        // Enable machine fast software interrupt
        mie::set_mfast_17();

        // SET machine software interrupt number 16 in ITC PULP IRQ MASK register
        let mask_set_reg: *mut u32 = (ITC_BASE_ADDR + ITC_MASK_SET_OFFSET) as *mut u32;
        write_volatile(mask_set_reg, 1 << SW_INTERRUPT_ID_16);

        // Enable global machine interrupts
        mstatus::set_mie();

        let write_value = 42;
        udma_uart::sprintln!("Core 0: Writing value {} to shared memory.", write_value);
        write_volatile(SHARED_MEM_ADDR as *mut u32, write_value);

        // Trigger the machine fast software interrupt number 16
        let interrupt_set_reg: *mut u32 = (ITC_BASE_ADDR + ITC_INT_SET_OFFSET) as *mut u32;
        write_volatile(interrupt_set_reg, 1 << SW_INTERRUPT_ID_16);
    }

    loop {}
}
