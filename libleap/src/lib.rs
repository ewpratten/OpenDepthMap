#[macro_use]
extern crate num_derive;

use std::{sync::{
        mpsc::{self, Sender},
        Arc,
    }, thread::Thread, time::{Duration, SystemTime}};

use error::LeapError;
use event::Event;
use leap_sys::{
     _eLeapRS_eLeapRS_Success,
    LEAP_CONNECTION, LEAP_CONNECTION_MESSAGE,LeapCreateConnection, LeapOpenConnection, LeapPollConnection
};
// use ffi::{LeapCreateConnection, LeapOpenConnection, LeapPollConnection};

pub mod error;
pub mod event;
// pub mod ffi;

// Cheaty way to ensure only one LeapDevice is created
static mut LEAP_DEVICE_EXISTS: bool = false;

/// Wrapper around a LeapMotion device
pub struct LeapDevice {
    /// Device handle
    handle: LEAP_CONNECTION,

    /// Storage for incoming messages
    latest_message: *mut LEAP_CONNECTION_MESSAGE,

    /// Weather the device is connected
    connected: bool,
}

impl LeapDevice {

    /// Open a connection to the first found LeapMotion device and wait for a connection or timeout
    pub fn new(timeout: Duration) -> Result<Self, LeapError> {

        // Connect to the device
        let mut device = Self::new_raw()?;

        // Track the start time
        let start = SystemTime::now();
        loop {

            // Handle timeout
            if start.elapsed().unwrap() > timeout {
                return Err(LeapError::Timeout);
            }

            // Poll for an event and let the handler do its work
            let _ = device.fetch_event();

        }

        Ok(device)

    }

    /// Open a connection to the first found LeapMotion device
    pub fn new_raw() -> Result<Self, LeapError> {
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
                connected: false
            })
        }
    }



    /// Fetch the latest event from the device
    pub fn fetch_event(&mut self) -> Result<Event, LeapError> {
        // Poll for a new message
        let result;
        unsafe {
            result = LeapPollConnection(self.handle, 1000, self.latest_message);
        }
        if result != _eLeapRS_eLeapRS_Success {
            return Err(result.into());
        }

        // Deref and convert into an event
        let event: Event;
        unsafe {
            event = (*self.latest_message).into();
        }

        // Handle checking connection status
        match event {
            Event::Connection(_) => self.connected = true,
            Event::ConnectionLost(_) => self.connected = false,
            _ => {}
        };

        Ok(event)
    }
}

impl Drop for LeapDevice {
    fn drop(&mut self) {
        todo!()
    }
}
