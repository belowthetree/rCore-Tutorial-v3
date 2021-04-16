
use virtio_drivers::{VirtIOBlk, VirtIOHeader};
use crate::{mm::{
    PhysAddr,
    VirtAddr,
    frame_alloc,
    frame_dealloc,
    PhysPageNum,
    FrameTracker,
    StepByOne,
    PageTable,
    kernel_token,
}};
use super::BlockDevice;
use spin::Mutex;
use alloc::vec::Vec;
use lazy_static::*;

#[allow(unused)]
const VIRTIO0: usize = 0x10001000;

pub struct VirtIOBlock(VirtIOBlk<'static>);

lazy_static! {
    static ref QUEUE_FRAMES: Mutex<Vec<FrameTracker>> = Mutex::new(Vec::new());
}

impl BlockDevice for VirtIOBlock {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        unsafe {
            let t = self as *const Self as *mut Self;
            let t = &mut (*t).0;
            t.read_block(block_id, buf).expect("Error when reading VirtIOBlk");
        }
    }
    fn write_block(&self, block_id: usize, buf: &[u8]) {
        unsafe {
            let t = self as *const Self as *mut Self;
            let t = &mut (*t).0;
            t.write_block(block_id, buf).expect("Error when writing VirtIOBlk");
        }
    }
    fn pending(&self, pin_idx : usize) {
        if pin_idx != 1 {
            return;
        }
        unsafe {
            let t = self as *const Self as *mut Self;
            let t = &mut (*t).0;
            let rt = t.pending();
            match rt{
                Ok(_) => {}
                Err(_) => {noteln!("Pending Err!!!")}
            }
        }
    }
}

impl VirtIOBlock {
    #[allow(unused)]
    pub fn new() -> Self {
        println!("new blk");
        let rt = Self(VirtIOBlk::new(
            unsafe { &mut *(VIRTIO0 as *mut VirtIOHeader) }
        ).unwrap());
        // let input = crate::drivers::input::virtio_input::VirtIOInputImpl::new();
        rt
    }
}

#[no_mangle]
pub extern "C" fn virtio_dma_alloc(pages: usize) -> PhysAddr {
    let mut ppn_base = PhysPageNum(0);
    for i in 0..pages {
        let frame = frame_alloc().unwrap();
        if i == 0 { ppn_base = frame.ppn; }
        assert_eq!(frame.ppn.0, ppn_base.0 + i);
        QUEUE_FRAMES.lock().push(frame);
    }
    // println!("virtio alloc {} pages {:x}", pages, ppn_base.0 * 4096);
    ppn_base.into()
}

#[no_mangle]
pub extern "C" fn virtio_dma_dealloc(pa: PhysAddr, pages: usize) -> i32 {
    let mut ppn_base: PhysPageNum = pa.into();
    for _ in 0..pages {
        frame_dealloc(ppn_base);
        ppn_base.step();
    }
    // println!("virtio dealloc {} pages {:x}", pages, pa.0);
    0
}

#[no_mangle]
pub extern "C" fn virtio_phys_to_virt(paddr: PhysAddr) -> VirtAddr {
    VirtAddr(paddr.0)
}

#[no_mangle]
pub extern "C" fn virtio_virt_to_phys(vaddr: VirtAddr) -> PhysAddr {
    PageTable::from_token(kernel_token()).translate_va(vaddr).unwrap()
}
