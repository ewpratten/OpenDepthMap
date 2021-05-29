use std::time::Duration;

use cli::logging::start_fern;
use libodm::leapmotion::device::LeapMotionDevice;

mod cli;

fn main() {

    start_fern();
    
    // Init a device
    let device = LeapMotionDevice::create_device(Duration::from_secs(1)).unwrap();

    log::info!("Got data");

}
