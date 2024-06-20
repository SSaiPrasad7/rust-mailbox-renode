#![no_std]
#![no_main]

use core::arch::asm;
use core::ptr;
use panic_halt as _;
use riscv_rt::entry;
use udma_uart;

#[entry]
fn main() -> ! {
    let baud_rate = 115200;
    const UART_BASE_ADDR: usize = 0x1A10_2100;
    const SHARED_MEM_ADDR: usize = 0x1C08_0000;

    use udma_uart::Uart;
    let _uart = Uart;
    udma_uart::udma_uart_init(UART_BASE_ADDR, baud_rate);

    for _ in 0..1000000 {
        unsafe { asm!("nop") }
    }

    unsafe {
        let value = ptr::read_volatile((SHARED_MEM_ADDR) as *mut u32);
        udma_uart::sprintln!("Core 1: Reading value {} from shared memory", value);
    }

    loop {}
}
