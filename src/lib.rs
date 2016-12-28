// TODO do w/out the unions?
#![feature(untagged_unions)]

mod ffi;

// pub mod cuda;

#[cfg(test)]
mod tests {
    use super::ffi::cuda::*;
    #[test]
    fn init_and_version() {
        let ret = unsafe { cuInit(0) };
        println!("{:?}", ret);

        let ver = unsafe {
            let mut ver = 0;
            cuDriverGetVersion(&mut ver as *mut i32);
            ver
        };

        println!("Version {}", ver);
    }
}
