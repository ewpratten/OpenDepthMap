use glam::UVec2;
use opencv::{core::{Image2D, Mat, Mat_AUTO_STEP, Size, CV_8U}, highgui::{imshow, named_window, wait_key}};

pub struct Image {
    size: UVec2,
    raw_data: *const u8,
    img: Mat,
}

impl Image {

    /// Create a new image from raw data
    pub fn new_raw(width: i32, height: i32, raw_data: *const u8) -> Result<Self, opencv::Error> {
        // Build a CV size
        let size = Size::new(width, height);

        let image;
        unsafe {
            image =
                Mat::new_size_with_data(size, CV_8U, raw_data as *mut libc::c_void, Mat_AUTO_STEP)?;
        }

        Ok(Self {
            size: UVec2::new(width as u32, height as u32),
            raw_data,
            img: image,
        })
    }

    /// Get the image size
    pub fn get_size(&self) -> &UVec2 {
        &self.size
    }

    /// Get the internal OpenCV image
    pub fn get_image(&self) -> &Mat {
        &self.img
    }

    /// Show this image in a debug window
    pub fn debug_show(&self){

        // Create a debug window
        imshow("Debug", &self.img).unwrap();
        wait_key(0).unwrap();

    }
}

// impl Drop for Image {
//     fn drop(&mut self) {
//         unsafe {
//             libc::free(self.raw_data as *mut libc::c_void);
//         }
//     }
// }
