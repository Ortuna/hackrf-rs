extern crate libc;
mod ffi;

pub const SUCCESS: i32 = ffi::hackrf_error::HACKRF_SUCCESS as i32;

pub struct HackRF {}
impl HackRF {

    pub fn new() -> HackRF {
        HackRF::init();
        HackRF{}
    }

    fn init() -> i32 {
        let result: i32;
        unsafe { result = ffi::hackrf_init() }

        HackRF::detect_error(result)
    }

    fn detect_error(result: i32) -> i32 {
        match result {
            result if result == SUCCESS => result,
            _ => panic!("ERROR: {}", result)
        }
    }
}

impl Drop for HackRF {
    fn drop(&mut self) {
        let result: i32;
        unsafe { result = ffi::hackrf_exit() }

        HackRF::detect_error(result);
    }
}

