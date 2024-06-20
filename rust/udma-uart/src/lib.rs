#![no_std]

use core::ptr;

const UART_TX_SADDR_OFFSET: usize = 0x10;
const UART_TX_SIZE_OFFSET: usize = 0x14;
const UART_TX_CFG_OFFSET: usize = 0x18;
const UART_SETUP_OFFSET: usize = 0x24;

const UART_BASE_ADDR: usize = 0x1a10_2100;

pub static mut SHARED_UART_ADDR: usize = UART_BASE_ADDR;

pub fn udma_uart_init(base_addr: usize, baud: u32) {
    let clk_freq = 30000000;

    let uart_clk_div = clk_freq / baud;
    let uart_config = (uart_clk_div << 16) | 0x317;

    unsafe {
        ptr::write_volatile((base_addr + UART_SETUP_OFFSET) as *mut u32, uart_config);
        SHARED_UART_ADDR = base_addr;
    };
}


#[inline]
pub fn udma_uart_write(s: &str) {
    unsafe {
        ptr::write_volatile(
            (SHARED_UART_ADDR + UART_TX_SADDR_OFFSET) as *mut u32,
            s.as_ptr() as u32,
        );
        ptr::write_volatile(
            (SHARED_UART_ADDR + UART_TX_SIZE_OFFSET) as *mut u32,
            s.len() as u32,
        );
        ptr::write_volatile((SHARED_UART_ADDR + UART_TX_CFG_OFFSET) as *mut u32, 1 << 4);
        
        // ???: Skip the wait on VP. This won't work on ASIC.
        // Wait for done
        //while ptr::read_volatile((SHARED_UART_ADDR + UART_TX_SADDR_OFFSET) as *mut u32) != 0 {}
    }
}

#[derive(Debug)]
pub struct Uart;

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        udma_uart_write(s);
        Ok(())
    }
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
