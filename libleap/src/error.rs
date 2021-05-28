use leap_sys::eLeapRS;

#[derive(FromPrimitive)]
pub enum LeapError {
    /// The operation completed successfully.
    Success = 0,
    /// An undetermined error has occurred.
    /// This is usually the result of an abnormal operating condition in LeapC,
    /// the Leap Motion service, or the host computer itself.
    UnknownError = -503250944,
    /// An invalid argument was specified.
    InvalidArgument = -503250943,
    /// Insufficient resources existed to complete the request.
    InsufficientResources = -503250942,
    /// The specified buffer was not large enough to complete the request.
    InsufficientBuffer = -503250941,
    /// The requested operation has timed out.
    Timeout = -503250940,
    /// The operation is invalid because there is no current connection.
    NotConnected = -503250939,
    /// The operation is invalid because the connection is not complete.
    HandshakeIncomplete = -503250938,
    /// The specified buffer size is too large.
    BufferSizeOverflow = -503250937,
    /// A communications protocol error occurred.
    ProtocolError = -503250936,
    /// The server incorrectly specified zero as a client ID.
    InvalidClientID = -503250935,
    /// The connection to the service was unexpectedly closed while reading or writing a message.
    /// The server may have terminated.
    UnexpectedClosed = -503250934,
    /// The specified request token does not appear to be valid
    ///
    /// Provided that the token value which identifies the request itself was, at one point, valid, this
    /// error condition occurs when the request to which the token refers has already been satisfied or
    /// is currently being satisfied.
    UnknownImageFrameRequest = -503250933,
    /// The specified frame ID is not valid or is no longer valid
    ///
    /// Provided that frame ID was, at one point, valid, this error condition occurs when the identifier
    /// refers to a frame that occurred further in the past than is currently recorded in the rolling
    /// frame window.
    UnknownTrackingFrameID = -503250932,
    /// The specified timestamp references a future point in time
    ///
    /// The related routine can only operate on time points having occurred in the past, and the
    /// provided timestamp occurs in the future.
    RoutineIsNotSeer = -503250931,
    /// The specified timestamp references a point too far in the past
    ///
    /// The related routine can only operate on time points occurring within its immediate record of
    /// the past.
    TimestampTooEarly = -503250930,
    /// LeapPollConnection is called concurrently.
    ConcurrentPoll = -503250929,
    /// A connection to the Leap Motion service could not be established.
    NotAvailable = -419364862,
    /// The requested operation can only be performed while the device is sending data.
    NotStreaming = -419364860,
    /// The specified device could not be opened. It is possible that the device identifier
    /// is invalid, or that the device has been disconnected since being enumerated.
    CannotOpenDevice = -419364859,
}

impl From<eLeapRS> for LeapError {
    fn from(error: eLeapRS) -> Self {
        match num::FromPrimitive::from_i32(error) {
            Some(e) => e,
            None => Self::UnknownError,
        }
    }
}
