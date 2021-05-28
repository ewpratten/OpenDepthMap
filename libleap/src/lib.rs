#[macro_use]
extern crate num_derive;

use std::{
    sync::{
        mpsc::{self, Sender},
        Arc,
    },
    thread::Thread,
};

use error::LeapError;
use event::Event;
use leap_sys::{
    LeapCreateConnection, LeapOpenConnection, LeapPollConnection, _eLeapRS_eLeapRS_Success,
    LEAP_CONNECTION, LEAP_CONNECTION_MESSAGE,
};

pub mod error;
pub mod event;

// Cheaty way to ensure only one LeapDevice is created
static mut LEAP_DEVICE_EXISTS: bool = false;

/// Wrapper around a LeapMotion device
pub struct LeapDevice {
    /// Device handle
    handle: LEAP_CONNECTION,

    /// Storage for incoming messages
    latest_message: *mut LEAP_CONNECTION_MESSAGE,
}

impl LeapDevice {
    pub fn new() -> Result<Self, LeapError> {
        // Handle being called too many times
        unsafe {
            if LEAP_DEVICE_EXISTS {
                panic!("Tried to call LeapDevice::new() more than once!");
            }
        }

        // Create a connection handle
        let handle = std::ptr::null_mut();

        // Try to create a connection
        let result;
        unsafe {
            result = LeapCreateConnection(std::ptr::null(), handle);
        }
        if result != _eLeapRS_eLeapRS_Success {
            return Err(result.into());
        }

        // Try to open a connection
        let result;
        unsafe {
            result = LeapOpenConnection(*handle);
        }
        if result != _eLeapRS_eLeapRS_Success {
            return Err(result.into());
        }

        // Set the device flag
        unsafe {
            LEAP_DEVICE_EXISTS = true;
        }

        unsafe {
            Ok(Self {
                handle: *handle,
                latest_message: std::ptr::null_mut(),
            })
        }
    }

    pub fn fetch_event(&mut self) -> Result<Event, LeapError> {
        // Poll for a new message
        let result;
        unsafe {
            result = LeapPollConnection(self.handle, 1000, self.latest_message);
        }
        if result != _eLeapRS_eLeapRS_Success {
            return Err(result.into());
        }

        unsafe{
            Ok((*self.latest_message).into())
        }
    }
}

impl Drop for LeapDevice {
    fn drop(&mut self) {
        todo!()
    }
}
