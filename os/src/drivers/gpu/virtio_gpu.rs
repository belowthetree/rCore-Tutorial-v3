use virtio_drivers::{VirtIOGpu, VirtIOHeader, DeviceType};
use core::intrinsics::transmute;

pub struct GPU(VirtIOGpu<'static>);

impl GPU {
    pub fn new()->Self {
        for i in (0x1000_1000..=0x1000_8000).step_by(0x1000) {
            let h = unsafe {&mut *(i as *mut VirtIOHeader)};
            if h.device_type() == DeviceType::GPU {
                println!("gpu at {:x}", i);
                let gpu = VirtIOGpu::new(h).unwrap();
                println!("gpu setup success");
                // gpu.
                return GPU(gpu);
            }
        }
        panic!("no gpu");
    }
}

impl GpuDrive for GPU {
    fn reset(&mut self) {
        self.0.setup_framebuffer().unwrap();
        self.0.flush().unwrap();
        println!("gpu resulotion {:?}", self.0.resolution());
    }

    fn draw_point(&mut self, x : usize, y : usize) {
        let buf : [u8;4] = [255;4];
        let b = unsafe {transmute::<[u8;4], [u32;1]>(buf)};
        self.0.draw_rect(x, y, 1, 1, &b);
    }

    fn refresh(&mut self) {
        self.0.flush().unwrap();
    }

    fn pending(&mut self) {
    }
    fn draw_test(&mut self) {
        let buf : [u8;400] = [255;400];
        let b = unsafe {transmute::<[u8;400], [u32;100]>(buf)};
        for x in 100..200 {
            self.0.draw_rect(x, 100, 10, 10, &b);
            self.0.draw_rect(x, 200, 10, 10, &b);
            self.0.draw_rect(200, x, 10, 10, &b);
            self.0.draw_rect(100, x, 10, 10, &b);
        }
    }
}

unsafe impl Send for GPU {}

pub trait GpuDrive : Send {
    fn pending(&mut self);
    fn reset(&mut self);
    fn draw_point(&mut self, x : usize, y : usize);
    fn refresh(&mut self);
    fn draw_test(&mut self); // 记得删除
}