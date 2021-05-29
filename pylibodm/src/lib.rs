use libodm::leapmotion::device::LeapMotionDevice;

pub(crate) static mut DEVICE_INSTANCE: Option<LeapMotionDevice> = None;

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

use std::time::Duration;

use libodm::{image::Image, leapmotion::device::DeviceFrame};
use pyo3::wrap_pyfunction;

#[pyclass]
#[derive(Debug, Clone)]
struct PyImage {
    #[pyo3(get)]
    pub width: u32,
    #[pyo3(get)]
    pub height: u32,
    #[pyo3(get)]
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
    #[pyo3(get)]
    pub bytes_per_pixel: u8,
    #[pyo3(get)]
    pub left_camera: PyImage,
    #[pyo3(get)]
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

#[pyfunction]
fn connect(timeout_secs: u64) -> Result<(), PyDeviceError> {
    // Create a leap device
    let device = LeapMotionDevice::create_device(Duration::from_secs(timeout_secs))?;

    // Override the instance
    unsafe {
        DEVICE_INSTANCE = Some(device);
    }

    Ok(())
}

#[pyfunction]
fn set_cameras_flipped(flipped: bool) {
    unsafe {
        DEVICE_INSTANCE
            .as_mut()
            .unwrap()
            .set_cameras_flipped(flipped);
    }
}

#[pyfunction]
fn get_cameras_flipped() -> bool {
    unsafe { DEVICE_INSTANCE.as_ref().unwrap().cameras_flipped() }
}

#[pyfunction]
fn has_frame() -> bool {
    unsafe { DEVICE_INSTANCE.as_mut().unwrap().has_frame() }
}

#[pyfunction]
fn get_frame(py: Python) -> Result<PyDeviceFrame, PyDeviceError> {
    Ok(unsafe { DEVICE_INSTANCE.as_mut().unwrap().get_frame()?.into_py(py) })
}

/// Python wrapper for libodm
#[pymodule]
fn libpylibodm(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    m.add_function(wrap_pyfunction!(get_frame, m)?)?;
    m.add_function(wrap_pyfunction!(has_frame, m)?)?;
    m.add_function(wrap_pyfunction!(get_cameras_flipped, m)?)?;
    m.add_function(wrap_pyfunction!(set_cameras_flipped, m)?)?;
    m.add_function(wrap_pyfunction!(connect, m)?)?;
    Ok(())
}
