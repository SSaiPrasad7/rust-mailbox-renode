#![no_std]
#![no_main]

use panic_halt as _;

use core::ptr;
use riscv_rt::entry;

const UART_BASE_ADDR: usize = 0x1a10_2100;
const UART_TX_SADDR_OFFSET: usize = 0x10;
const UART_TX_SIZE_OFFSET: usize = 0x14;
const UART_TX_CFG_OFFSET: usize = 0x18;
const UART_SETUP_OFFSET: usize = 0x24;

fn uart_init(baud: u32) {
    let clk_freq = 30000000;

    let uart_clk_div = clk_freq / baud;
    let uart_config = (uart_clk_div << 16) | 0x317;

    unsafe {
        ptr::write_volatile(
            (UART_BASE_ADDR + UART_SETUP_OFFSET) as *mut u32,
            uart_config,
        )
    };
}

#[derive(Debug)]
struct Uart;

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        uart_write(s);
        Ok(())
    }
}

#[inline]
pub fn uart_write(s: &str) {
    unsafe {
        ptr::write_volatile(
            (UART_BASE_ADDR + UART_TX_SADDR_OFFSET) as *mut u32,
            s.as_ptr() as u32,
        );
        ptr::write_volatile(
            (UART_BASE_ADDR + UART_TX_SIZE_OFFSET) as *mut u32,
            s.len() as u32,
        );
        ptr::write_volatile((UART_BASE_ADDR + UART_TX_CFG_OFFSET) as *mut u32, 1 << 4);

        // ???: Skip the wait on VP. This won't work on ASIC.
        // Wait for done
        //while ptr::read_volatile((UART_BASE_ADDR + UART_TX_SADDR_OFFSET) as *mut u32) != 0 {}
    }
}

#[macro_export]
macro_rules! sprintln {
    () => {{
        use $crate::sprint;
        sprint!("\r\n");
    }};
    // IMPORTANT use `tt` fragments instead of `expr` fragments (i.e. `$($exprs:expr),*`)
    ($($tt:tt)*) => {{
        use $crate::sprint;
        sprint!($($tt)*);
        sprint!("\r\n");
    }};
}

#[macro_export]
macro_rules! sprint {
    ($s:expr) => {{
        use core::fmt::Write;
        write!(Uart {}, $s).unwrap()
    }};
    ($($tt:tt)*) => {{
        use core::fmt::Write;
        write!(Uart, $($tt)*).unwrap()
    }};
}

#[entry]
fn main() -> ! {
    let baud_rate = 115200;
    uart_init(baud_rate);

    let s = "\r\nHello World\r\n";
    uart_write(s);
    uart_write(s);

    sprintln!("Hello {}", "world");
    sprintln!("Here's some numbers {}, {}, {}", 1, 2, 3);

    sprintln!("What's the UART doing? {:?}", Uart {});

    loop {}
}
