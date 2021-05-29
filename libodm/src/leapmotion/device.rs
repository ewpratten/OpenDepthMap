use crate::leapmotion::ffi::endEventLoop;

use super::ffi;

#[derive(Debug)]
pub enum DeviceError {
    InstanceError,
}

pub struct LeapMotionDevice {}

impl LeapMotionDevice {

    /// Creates a LeapMotionDevice by talking to the device driver. This can only be called once
    pub fn create_device() -> Result<Self, DeviceError> {
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

        log::debug!("Device created");
        Ok(Self {})
    }
}

impl Drop for LeapMotionDevice {
    fn drop(&mut self) {
        log::debug!("Informing wrapper to deallocate LeapMotion device");
        unsafe{
            ffi::endEventLoop();
        }
    }
}