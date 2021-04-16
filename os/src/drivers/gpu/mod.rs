mod virtio_gpu;


use alloc::sync::Arc;
use lazy_static::*;
use spin::Mutex;
use virtio_gpu::GPU;

use self::virtio_gpu::GpuDrive;

lazy_static! {
    pub static ref GPU_DEVICE : Arc<Mutex<dyn GpuDrive>> = Arc::new(Mutex::new(GPU::new()));
}