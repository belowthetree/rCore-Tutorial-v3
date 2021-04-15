pub mod input;
mod block;

pub use block::BLOCK_DEVICE;

pub fn pending(pin_idx : usize) {
    block::pending(pin_idx);
    input::pending(pin_idx);
}