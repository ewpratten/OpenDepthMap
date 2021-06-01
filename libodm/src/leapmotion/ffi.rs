//! FFI definitions for LeapMotion wrapper code

extern {

    /// Begin the device event loops, init the device, and push policy changes
    pub fn beginEventLoop();

    /// Check if the controller object has been created
    pub fn isControllerCreated() -> bool;

    /// End everything and clean up
    pub fn endEventLoop();

    /// Poll for new data
    pub fn updateFrame();

    /// Check if the device has a pair of camera images to work with
    pub fn imageExists() -> bool;

    /// Get the camera image height
    pub fn getImageHeight() -> libc::c_int;

    /// Get the camera image width
    pub fn getImageWidth()-> libc::c_int;

    /// Get the number of bytes used to represent a single pixel (should always be 1)
    pub fn getImageBPP() -> libc::c_int;

    /// Get a pointer to the left image pixel data
    pub fn getImageLeft() -> *const u8;

    /// Get a pointer to the right image pixel data
    pub fn getImageRight()  -> *const u8;
}