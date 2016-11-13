extern crate hackrf;

use hackrf::HackRF;

#[test]
fn it_doesnt_panic_on_create() {
    let _x = HackRF::new();
}

