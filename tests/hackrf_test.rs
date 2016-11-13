extern crate hackrf;

use hackrf::HackRF;

#[test]
fn it_can_init_and_exit() {
    assert_eq!(hackrf::SUCCESS, HackRF::init());
    assert_eq!(hackrf::SUCCESS, HackRF::exit());
}

