#![allow(dead_code)]
use crate::task::{current_task};
use alloc::vec::Vec;

pub struct Condvar<T : PartialEq> {
    wait_queue : spin::Mutex<Vec<(usize, T)>>,
}

impl<T : PartialEq> Condvar<T> {
    pub fn new()->Self {
        Self {
            wait_queue : spin::Mutex::new(Vec::new())
        }
    }

    pub fn wait(&self, condition : T) {
        self.wait_queue.lock().push((current_task().unwrap().pid.0, condition))
    }

    pub fn notify(&self, f : impl Fn(&T)->bool) {
        if let Some((_pid, _)) = self.wait_queue.lock().iter().find(|c| {
            f(&c.1)
        }) {
            // TODO wakeup pid
        }
    }
}