use std::time::Duration;

use libodm::leapmotion::device::LeapMotionDevice;

fn main() {
    
    // Init a device
    let device = LeapMotionDevice::create_device().unwrap();

    std::thread::sleep(Duration::from_secs(1));
}
