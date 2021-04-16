//! # PLIC
//! 管理时间中断以及外部设备中断，这里只负责 1 ~ 10 的外部中断
//! 外部设备（包括块设备、uart等）通过 plic 控制
//! 接收中断（handler）->获取中断来源针脚（claim）->通知处理完毕（complete）
//! 2020年12月 zg

#![allow(dead_code)]

use crate::{drivers, uart};

const PENDING      : usize = 0x0c00_1000;
const ENABLE       : usize = 0x0c00_2000;
const PRIORITY     : usize = 0x0c00_0000;
const THRESHOLD    : usize = 0x0c20_0000;
const CLAIM        : usize = 0x0c20_0004; // Read
const COMPLETE     : usize = 0x0c20_0004; // Write

pub fn init(){
    set_threshold(0);
    for i in 1..=10{
        enable(i);
        set_priority(i, 7);
    }
}

fn complete(idx : u32){
    unsafe {
        let ptr = COMPLETE as *mut u32;
        ptr.write_volatile(idx);
    }
}

fn claim() -> Option<u32> {
    unsafe {
        let ptr = CLAIM as *mut u32;
        let idx = ptr.read_volatile();
        if idx == 0 {
            None
        }
        else{
            Some(idx)
        }
        
    }
}

// #[allow(dead_code)]
// fn pending() -> Option<u32> {
//     unsafe {
//         let ptr = PENDING as *mut u32;
//         let idx = ptr.read_volatile();
//         if idx == 0{
//             None
//         }
//         else {
//             Some(idx)
//         }
//     }
// }

fn set_threshold(num : usize){
    unsafe {
        let ptr = THRESHOLD as *mut u32;
        ptr.write_volatile((num & 7) as u32);
    }
}

fn enable(idx : usize){
    unsafe {
        let ptr = ENABLE as *mut u32;
        ptr.write_volatile(ptr.read_volatile() | 1 << idx);
    }
}

fn set_priority(idx : usize, num : usize){
    unsafe {
        let ptr = PRIORITY as *mut u32;
        ptr.add(idx).write_volatile(num as u32);
    }
}

pub fn pending(){
    if let Some(pin) = claim(){
        match pin {
            1..=8 => {
                drivers::pending(pin as usize);
            }
            10 => {
                uart::pending();
            }
            _ => {
                panic!("Unknown pin interrupt");
            }
        }
        complete(pin);
    }
}

