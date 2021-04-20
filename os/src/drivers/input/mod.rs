#![allow(dead_code)]
pub mod virtio_input;

use core::any::Any;

use alloc::sync::Arc;
use lazy_static::*;
use virtio_input::VirtIOInputImpl;

lazy_static! {
    pub static ref INPUT_DEVICE : Arc<dyn InputDevice> = Arc::new(VirtIOInputImpl::new());
}

pub trait InputDevice : Send + Sync + Any {
    fn pending(&self, pin_idx : usize);
}

pub fn pending(pin_idx : usize) {
    // 取消注释后会引发错误
    // INPUT_DEVICE.clone().pending(pin_idx);
}