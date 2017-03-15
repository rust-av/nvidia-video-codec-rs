use std::os::raw::c_int;
use std::os::raw::c_char;

use ffi::cuda::*;
use ffi::cuda::cudaError_enum::*;

pub struct CuDevice {
    device: CUdevice,
}

impl CuDevice {
    pub fn new(ordinal: c_int) -> Result<CuDevice, CUresult> {
        let mut d = CuDevice { device: 0 };
        let res = unsafe { cuDeviceGet(&mut d.device as *mut i32, ordinal) };

        wrap!(d, res)
    }

    pub fn get_attribute(&self, attr: CUdevice_attribute) -> Result<c_int, CUresult> {
        let mut pi = 0;
        let res = unsafe { cuDeviceGetAttribute(&mut pi as *mut i32, attr, self.device) };

        wrap!(pi, res)
    }

    pub fn get_name(&self) -> Result<String, CUresult> {
        let mut name = vec![0; 256];
        let res = unsafe {
            cuDeviceGetName(name.as_mut_ptr() as *mut c_char,
                            name.len() as i32,
                            self.device)
        };
        let val = String::from_utf8(name).unwrap();

        wrap!(val, res)
    }

    pub fn get_total_mem(&self) -> Result<usize, CUresult> {
        let mut val = 0;
        let res = unsafe { cuDeviceTotalMem_v2(&mut val as *mut usize, self.device) };

        wrap!(val, res)
    }
}

pub fn get_count() -> Result<c_int, CUresult> {
    let mut val = 0;
    let res = unsafe { cuDeviceGetCount(&mut val as *mut i32) };

    wrap!(val, res)
}

#[cfg(test)]
mod tests {
    use ffi::cuda::cuInit;
    use super::*;

    #[test]
    fn device_enum() {
        let _ = unsafe { cuInit(0) };

        for i in 0..get_count().unwrap() {
            let dev = CuDevice::new(i).unwrap();

            println!("{} {}",
                     dev.get_name().unwrap(),
                     dev.get_total_mem().unwrap());
        }
    }
}
