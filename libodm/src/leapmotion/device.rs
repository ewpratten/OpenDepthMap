//! Defines an interface for LeapMotion cameras

use std::time::{Duration, SystemTime};

use crate::image::Image;

use super::ffi;

/// Defines some errors that can come from the device
#[derive(Debug)]
pub enum DeviceError {
    /// An error caused by too many instances being created
    InstanceError,

    /// An error caused by a timeout being reached
    Timeout,

    /// An error caused when no data is being sent by the device
    NoData,
}

/// Represents a single data frame generated by the LeapMotion device
#[derive(Debug)]
pub struct DeviceFrame {
    /// Number of bytes per pixel
    pub bytes_per_pixel: u8,

    /// Image from the left camera
    pub left_camera: Image,

    /// Image from the right camera
    pub right_camera: Image,
}

/// Represents a LeapMotion device, and provides safe wrappers around its FFI
#[derive(Debug)]
pub struct LeapMotionDevice {
    cameras_flipped: bool,
}

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
        Ok(Self {
            cameras_flipped: false,
        })
    }

    /// Configure the camera to flip its cameras. In normal configuration, the green light is facing the user, while the device is laying flat
    pub fn set_cameras_flipped(&mut self, flipped: bool) {
        self.cameras_flipped = flipped;
    }

    /// Get if the cameras are flipped
    pub fn cameras_flipped(&self) -> bool {
        self.cameras_flipped
    }

    /// Get if the device has a frame to be read
    pub fn has_frame(&mut self) -> bool {
        unsafe {
            return ffi::isControllerCreated() && ffi::imageExists();
        }
    }

    /// Get the latest frame from the device
    pub fn get_frame(&mut self) -> Result<DeviceFrame, DeviceError> {
        if !self.has_frame() {
            return Err(DeviceError::NoData);
        }

        // Read raw data from the device
        let image_width;
        let image_height;
        let bytes_per_pixel;
        let left_image_raw;
        let right_image_raw;
        unsafe {
            image_width = ffi::getImageWidth();
            image_height = ffi::getImageHeight();
            bytes_per_pixel = ffi::getImageBPP();
            left_image_raw = std::slice::from_raw_parts(
                ffi::getImageLeft(),
                (image_width * image_height * bytes_per_pixel) as usize,
            );
            right_image_raw = std::slice::from_raw_parts(
                ffi::getImageRight(),
                (image_width * image_height * bytes_per_pixel) as usize,
            );
        }

        // Build two images
        let left_image = Image::new(image_width, image_height, left_image_raw);
        let right_image = Image::new(image_width, image_height, right_image_raw);

        Ok(DeviceFrame {
            bytes_per_pixel: bytes_per_pixel as u8,
            left_camera: left_image,
            right_camera: right_image,
        })
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
