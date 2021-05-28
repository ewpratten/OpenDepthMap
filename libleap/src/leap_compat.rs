//! Some cleanup and tweaks to the LeapMotion bindings
//!
//! Since this library binds directly to `leap-sys`, some code
//! is not exactly fun to use. This file contains a few small shorthands
//! and typedefs and stuff

pub mod types {
    use std::ops::{Deref, DerefMut};

    use leap_sys::LEAP_CONNECTION;

    /// A thin wrapper around `_LEAP_CONNECTION`
    pub struct LeapConnection(LEAP_CONNECTION);

    impl Deref for LeapConnection {
        type Target = LEAP_CONNECTION;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for LeapConnection {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl From<LEAP_CONNECTION> for LeapConnection {
        fn from(c: LEAP_CONNECTION) -> Self {
            Self { 0: c }
        }
    }
}

pub mod errors {
    use leap_sys::eLeapRS;

    /// A generic error, wrapping LeapMotion's `eLeapRS`
    #[derive(Debug)]
    pub struct GenericLeapError {
        pub error: eLeapRS,
    }

    impl From<eLeapRS> for GenericLeapError {
        fn from(error: eLeapRS) -> Self {
            Self { error }
        }
    }
}

pub mod wrappers {
    use leap_sys::{LeapCreateConnection, _eLeapRS_eLeapRS_Success};

    use super::{errors::GenericLeapError, types::LeapConnection};

    pub unsafe fn create_connection() -> Result<LeapConnection, GenericLeapError> {
        // Create a handle for the connection
        let connection_handle = std::ptr::null_mut();

        // Call the leap driver
        let result = LeapCreateConnection(std::ptr::null(), connection_handle);

        // Handle errors
        if result != _eLeapRS_eLeapRS_Success {
            return Err(result.into());
        }

        return Ok((*connection_handle).into());
    }
}
