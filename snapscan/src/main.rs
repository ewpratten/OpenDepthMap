use std::time::Duration;

use clap::{value_t, App, Arg};
use libleap::LeapDevice;

fn main() {
    let matches = App::new("snapscan")
        .author("Evan Pratten <ewpratten@gmail.com>")
        .get_matches();

        // Open a connection to the device 
        let device = LeapDevice::new(Duration::from_secs(1)).unwrap();

        println!("Connected to LeapMotion device");
}
