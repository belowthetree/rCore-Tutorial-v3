mod block;

pub use block::BLOCK_DEVICE;

pub fn handler(pin_idx : usize) {
    block::handler(pin_idx);
}