use super::File;
use crate::mm::{UserBuffer};
use crate::sbi::console_getchar;
use crate::task::suspend_current_and_run_next;

pub struct Stdin;

pub struct Stdout;

impl File for Stdin {
    fn readable(&self) -> bool { true }
    fn writable(&self) -> bool { false }
    fn read(&self, mut user_buf: UserBuffer) -> usize {
        // println!("stdin");
        assert_eq!(user_buf.len(), 1);
        // busy loop
        let mut ch;
        loop {
            // c = console_getchar();
            match crate::uart::pop() {
                Some(c) => {
                    ch = c;
                    break;
                }
                _ => {
                    suspend_current_and_run_next();
                    continue;
                }
            }
        }
        unsafe { user_buf.buffers[0].as_mut_ptr().write_volatile(ch); }
        1
    }
    fn write(&self, _user_buf: UserBuffer) -> usize {
        panic!("Cannot write to stdin!");
    }
}

impl File for Stdout {
    fn readable(&self) -> bool { false }
    fn writable(&self) -> bool { true }
    fn read(&self, _user_buf: UserBuffer) -> usize{
        panic!("Cannot read from stdout!");
    }
    fn write(&self, user_buf: UserBuffer) -> usize {
        // println!("stdout");
        for buffer in user_buf.buffers.iter() {
            print!("{}", core::str::from_utf8(*buffer).unwrap());
        }
        // println!("after stdout");
        user_buf.len()
    }
}