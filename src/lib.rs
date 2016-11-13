extern crate libc;
mod ffi;

pub const SUCCESS: i32 = ffi::hackrf_error::HACKRF_SUCCESS as i32;

pub struct HackRF {}
impl HackRF {
    pub fn init() -> i32 {
        let result: i32;
        unsafe { result = ffi::hackrf_init() }

        HackRF::detect_error(result)
    }

    pub fn exit() -> i32 {
        let result: i32;
        unsafe { result = ffi::hackrf_exit() }

        HackRF::detect_error(result)
    }

    pub fn new() {

    }

    fn detect_error(result: i32) -> i32 {
        match result {
            result if result == SUCCESS => result,
            _ => panic!("ERROR: {}", result)
        }
    }
}

