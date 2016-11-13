use std::ffi::{CString, CStr};
use std::ptr;
use std::mem;

use ffi;

pub struct Device {
    pub board_id: ffi::hackrf_usb_board_id,
    pub serial_number: char,
    pub device_index: i32,
}

pub struct Devices{}

impl Devices {
    pub fn fetch() -> Vec<Device> {
        let mut devices = Vec::new();

        let list               = unsafe { ffi::hackrf_device_list() };
        let device_count       = unsafe { (*list).devicecount };

        for index in 0..device_count {
            devices.push(
                Device {
                    board_id:      Devices::extract_board_id(list, index),
                    serial_number: Devices::extract_serial_number(list, index),
                    device_index:  index,
                }
            )
        }

        unsafe { ffi::hackrf_device_list_free(list) };

        devices
    }

    fn extract_serial_number(list: *mut ffi::hackrf_device_list, index: i32) -> char {
        unsafe { *(*list).serial_numbers as u8 as char}
     }

    fn extract_board_id(list: *mut ffi::hackrf_device_list, index: i32) -> ffi::hackrf_usb_board_id {
        let ptr = unsafe { (*list).usb_board_ids };
        unsafe { *ptr.offset(index as isize) }
    }

}
