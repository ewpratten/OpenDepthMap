use std::time::{Duration, SystemTime};

use crate::leapmotion::ffi::endEventLoop;

use super::ffi;

#[derive(Debug)]
pub enum DeviceError {
    InstanceError,
    Timeout,
}

pub struct LeapMotionDevice {}

impl LeapMotionDevice {
    /// Creates a LeapMotionDevice by talking to the device driver. This can only be called once
    pub fn create_device(timeout: Duration) -> Result<Self, DeviceError> {
        log::debug!("Creating LeapMotion device access");

        // Handle possible error with multiple instances
        unsafe {
            if ffi::isControllerCreated() {
                log::error!(
                    "Tried to create access to a LeapMotion device when an instance already exists"
                );
                return Err(DeviceError::InstanceError);
            }
        }

        // Create an instance along with its handlers
        unsafe {
            ffi::beginEventLoop();
        }

        // Pause a small amount of time for the device to finish init or for the timeout to be hit
        let time_now = SystemTime::now();
        loop {
            // Handle timeout
            if time_now.elapsed().unwrap() > timeout {
                log::error!("Hit device init timeout");
                return Err(DeviceError::Timeout);
            }

            // Handle device connect
            unsafe {
                if ffi::isControllerCreated() && ffi::imageExists() {
                    break;
                }
            }
        }

        log::debug!("Device created");
        Ok(Self {})
    }
}

impl Drop for LeapMotionDevice {
    fn drop(&mut self) {
        log::debug!("Informing wrapper to deallocate LeapMotion device");
        unsafe {
            ffi::endEventLoop();
        }
    }
}
