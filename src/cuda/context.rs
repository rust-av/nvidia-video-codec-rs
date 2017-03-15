use ffi::cuda::*;

use cuda::device::CuDevice;

pub struct CuContext {
    context : CUcontext,
}

impl CuContext {
    pub fn new(dev : CuDevice, flags : u32) -> Result<CuContext, CUresult> {
        let mut ctx = CuContext { context : 0 };
        let res = unsafe { cuCtxCreate(&mut ctx.context as *mut CUcontext, dev.device) }

        wrap!(ctx, res)
    }

    pub fn get_api_version(&self) -> Result<u32, CUresult> {
        let mut ver = 0;
        let res = unsafe { cuGetApiVersion(self.context, &mut ver as *mut u32)};

        wrap!(ver, res)
    }

    pub fn pop() -> Result<, CUresult> {
        let res = unsafe { cuCtxPopCurrent
    }
}

/* TODO: leverage clone/drop/deref traits
impl Clone for CuContext {

}

impl Deref for CuContext {

}
*/

impl Drop for CuContext {
    fn drop(&mut self) {
        unsafe {
            cuCtxDestroy(self.context);
        }
    }
}
