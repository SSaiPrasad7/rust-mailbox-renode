#![no_std]
#![no_main]

use panic_halt as _;
use riscv_rt::entry;
use udma_uart;

#[entry]
fn main() -> ! {
    let baud_rate = 115200;
    const UART_BASE_ADDR: usize = 0x1a10_2100;
    use udma_uart::Uart;
    let _uart = Uart;

    udma_uart::udma_uart_init(UART_BASE_ADDR, baud_rate);

    udma_uart::sprintln!("Hello {}", "world");
    udma_uart::sprintln!("Here's some numbers {}, {}, {}", 1, 2, 3);
    loop {}
}
