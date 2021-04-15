use virtio_drivers::{DeviceType, VirtIOHeader, VirtIOInput};
use super::InputDevice;
use alloc::vec::Vec;

const INPUT_ADDR_START : usize = 0x1000_1000;
pub struct VirtIOInputImpl {
    driver : VirtIOInput<'static>,
    buffer : Vec<u64>,
}

unsafe impl Send for VirtIOInputImpl{}

impl VirtIOInputImpl {
    pub fn new()->Self {
        let mut buffer = Vec::<u64>::new();
        for i in 0..32 {
            buffer.push(0);
        }
        let t = buffer.as_mut_slice() as *mut [u64];
        let mut rt = None;
        for i in (INPUT_ADDR_START..(INPUT_ADDR_START + 0x8000)).step_by(0x1000) {
            let header = unsafe {&mut *(i as *mut VirtIOHeader)};
            if header.device_type() == DeviceType::Input {
                println!("{:x}", i);
                rt = Some(Self {
                    driver : VirtIOInput::new(header, unsafe {&mut *t}).unwrap(),
                    buffer
                });
                break;
            }
            else {
                println!("{:?}", header.device_type());
            }
        }
        println!("after new input");
        rt.unwrap()
    }
}

impl InputDevice for VirtIOInputImpl {
    fn pending(&self, pin_idx : usize) {
        if pin_idx != 8 {
            return;
        }
        unsafe {
            let t = self as *const Self as *mut Self;
            noteln!("{:?}", (*t).driver.pending());
        }
    }
}