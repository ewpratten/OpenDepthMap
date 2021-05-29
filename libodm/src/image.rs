
pub struct Image<'a> {
    width: u32,
    height: u32,
    buffer: &'a [u8],
}

impl Image<'_> {
    /// Create a new image from raw data
    pub fn new(width: i32, height: i32, raw_data: *const u8) -> Self {
        Self {
            width: width as u32,
            height: height as u32,
            buffer: unsafe { std::slice::from_raw_parts(raw_data, (width * height) as usize) },
        }
    }

    /// Get the image width
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Get the image height
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Get the internal OpenCV image
    pub fn get_image(&self) -> &[u8] {
        &self.buffer
    }
}
