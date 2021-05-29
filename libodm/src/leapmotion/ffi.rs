
extern {
    pub fn beginEventLoop();
    pub fn isControllerCreated() -> bool;
    pub fn endEventLoop();
    pub fn imageExists() -> bool;
    pub fn getImageHeight() -> libc::c_int;
    pub fn getImageWidth()-> libc::c_int;
    pub fn getImageBPP() -> libc::c_int;
    pub fn getImageLeft() -> *const u8;
    pub fn getImageRight()  -> *const u8;
}