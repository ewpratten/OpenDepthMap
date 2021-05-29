use libodm::leapmotion::device::LeapMotionDevice;

pub(crate) static mut DEVICE_INSTANCE: Option<LeapMotionDevice> = None;

mod errors {
    use libodm::leapmotion::device::DeviceError;
    use pyo3::{exceptions::PyOSError, prelude::*};

    #[derive(Debug)]
    pub struct PyDeviceError(DeviceError);

    impl From<DeviceError> for PyDeviceError {
        fn from(e: DeviceError) -> Self {
            Self { 0: e }
        }
    }

    impl std::convert::From<PyDeviceError> for PyErr {
        fn from(err: PyDeviceError) -> PyErr {
            PyOSError::new_err(format!("{:?}", err.0))
        }
    }
}

mod device {
    use std::time::Duration;

    use libodm::{
        image::Image,
        leapmotion::device::{DeviceFrame, LeapMotionDevice},
    };
    use pyo3::prelude::*;

    use crate::{errors::PyDeviceError, DEVICE_INSTANCE};

    #[pyclass]
    struct PyImage {
        pub width: u32,
        pub height: u32,
        pub buffer: Vec<u8>,
    }

    impl IntoPy<PyImage> for Image<'_> {
        fn into_py(self, py: Python) -> PyImage {
            PyImage {
                width: self.get_width(),
                height: self.get_height(),
                buffer: self.get_image().into_iter().map(|e| *e).collect(),
            }
        }
    }

    #[pyclass]
    struct PyDeviceFrame {
        pub bytes_per_pixel: u8,
        pub left_camera: PyImage,
        pub right_camera: PyImage,
    }

    impl IntoPy<PyDeviceFrame> for DeviceFrame<'_> {
        fn into_py(self, py: Python) -> PyDeviceFrame {
            PyDeviceFrame {
                bytes_per_pixel: self.bytes_per_pixel,
                left_camera: self.left_camera.into_py(py),
                right_camera: self.right_camera.into_py(py),
            }
        }
    }

    #[pymodule]
    pub fn leap_device(py: Python, m: &PyModule) -> PyResult<()> {
        #[pyfn(m, "connect")]
        fn connect(timeout_secs: u64) -> Result<(), PyDeviceError> {
            // Create a leap device
            let device = LeapMotionDevice::create_device(Duration::from_secs(timeout_secs))?;

            // Override the instance
            unsafe {
                DEVICE_INSTANCE = Some(device);
            }

            Ok(())
        }

        #[pyfn(m, "set_cameras_flipped")]
        fn set_cameras_flipped(flipped: bool) -> PyResult<()> {
            unsafe {
                DEVICE_INSTANCE
                    .as_mut()
                    .unwrap()
                    .set_cameras_flipped(flipped);
            }

            Ok(())
        }

        #[pyfn(m, "get_cameras_flipped")]
        fn get_cameras_flipped() -> PyResult<bool> {
            Ok(unsafe { DEVICE_INSTANCE.as_ref().unwrap().cameras_flipped() })
        }

        #[pyfn(m, "has_frame")]
        fn has_frame() -> PyResult<bool> {
            Ok(unsafe { DEVICE_INSTANCE.as_mut().unwrap().has_frame() })
        }

        #[pyfn(m, "get_frame")]
        fn get_frame(py: Python) -> Result<PyDeviceFrame, PyDeviceError> {
            Ok(unsafe { DEVICE_INSTANCE.as_mut().unwrap().get_frame()?.into_py(py) })
        }

        Ok(())
    }
}
