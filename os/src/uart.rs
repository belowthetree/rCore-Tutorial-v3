//! # uart.rs
//! UART 可以直接传输用户的输入输出
//! 作为前期输出调试的主要手段，必须优先实现
//! 
//! 2020年12月 zg

use core::fmt::{self, Write, Error};
use core::convert::TryInto;

pub const UART_ADDR : usize = 0x1000_0000;
const LSR_OFFSET : usize = 5;

struct InBuffer {
    buffer : [u8;128],
    read_idx : usize,
    write_idx : usize,
}

impl InBuffer {
    pub const fn new()->Self {
        Self {
            buffer : [0;128],
            read_idx : 0,
            write_idx : 0,
        }
    }

    pub fn pending(&mut self) {
        let c = Uart::new().get().unwrap();
        self.buffer[self.write_idx] = c;
        self.write_idx = (self.write_idx + 1) % 128;
    }

    pub fn pop(&mut self)->Option<u8> {
        if self.write_idx == self.read_idx { None }
        else {
            let rt = Some(self.buffer[self.read_idx]);
            self.read_idx = (self.read_idx + 1) % 128;
            rt
        }
    }
}

static mut BUFFER: InBuffer = InBuffer::new();

pub fn pending() {
    unsafe {
        BUFFER.pending();
    }
}

pub fn pop()->Option<u8> {
    unsafe {
        BUFFER.pop()
    }
}

pub struct Uart;
/// 继承 Write Trait 使得 print 宏得以使用
/// 字符转换等由 Rust 提供，非常方便
impl Write for Uart {
	fn write_str(&mut self, out: &str) -> Result<(), Error> {
		for c in out.bytes() {
			self.put(c);
		}
		Ok(())
	}
}

pub fn print(args: fmt::Arguments) {
    Uart.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! note {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::uart::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! noteln {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::uart::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

impl Uart {
    pub fn new() -> Self {
        Uart
    }
    /// ## 初始化 UART
    /// 主要包括：
    /// lcr：每次传输的数据位数
    /// fcr：先进先出
    /// 激活
    pub fn init(&mut self) {
        unsafe {
            let ptr = UART_ADDR as *mut u8;
            // 偏移 3 指出每次传输的位数，恒定 8 位即一字节
            ptr.add(3).write_volatile(8);
            // 激活 FIFI
            ptr.add(2).write_volatile(1);
            // 激活中断
            ptr.add(1).write_volatile(1);
            // 设置输入产生的中断频率
            let divisor : u16 = 592;
			let divisor_least: u8 = (divisor & 0xff).try_into().unwrap();
			let divisor_most:  u8 = (divisor >> 8).try_into().unwrap();
            let lcr = ptr.add(3).read_volatile();
            ptr.add(3).write_volatile(lcr | 1 << 7);
            
            ptr.add(0).write_volatile(divisor_least);
            ptr.add(1).write_volatile(divisor_most);
            ptr.add(3).write_volatile(lcr);
        }
    }
    /// ## 获取键盘输入
    /// 从 MMIO 对应地址获取输入
    pub fn get(&self) -> Option<u8> {
        unsafe {
            let ptr = UART_ADDR as *mut u8;
            if ptr.add(LSR_OFFSET).read_volatile() & 1 == 0 {
                None
            }
            else {
                Some(ptr.read_volatile())
            }
        }
    }
    /// ## 输出
    /// 通过 MMIO 的方式
    pub fn put(&mut self, c : u8) {
        unsafe {
            let ptr = UART_ADDR as *mut u8;
            ptr.add(0).write_volatile(c);
        }
    }
}

