extern crate hackrf;

use hackrf::HackRF;

#[test]
fn it_doesnt_panic_on_create() {
    let _rf = HackRF::new();
}

#[test]
fn it_can_list_devices() {
    let rf = HackRF::new();
    rf.list();
}

#[test]
fn it_can_open() {
    let rf = HackRF::new();
    rf.open();
    rf.close();
}

