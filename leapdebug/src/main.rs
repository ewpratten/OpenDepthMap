use std::time::Duration;

use cli::logging::start_fern;
use libodm::leapmotion::device::LeapMotionDevice;

mod cli;

fn main() {
    start_fern();

    // Init a device
    let mut device = LeapMotionDevice::create_device(Duration::from_secs(4)).unwrap();

    log::info!("Got data");

    log::info!("Press CTRL+C to exit");
    loop {
        // Read a single frame
        let frame = device.get_frame().unwrap();

        // Show data
        frame.debug_show();
    }
}
