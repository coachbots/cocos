/// This module exposes the [ApiError] enum which is used for all api-related
/// errors.

use zmq;

#[derive(Debug)]
/// Represents errors that occur during API transactions.
pub enum ApiError {
    General,
    ProcessError,
    IO,

    /// Raised when a timeout ocurred waiting for a message to be sent.
    TimeoutError,
    /// Raised when the socket is not in a usable state.
    SockNotReady,
    /// Represents ZMQ errors.
    ZMQError(zmq::Error),

    /// Raised when a message is received in bytes, but said bytes are not
    /// valid UTF-8.
    DecodeError,
    /// Raised when a message is received, but does not adhere to the valid
    /// standard form.
    InvalidRequestHead,
    /// Raised when a message is received correctly, but the body could not be
    /// decoded according to the given type.
    InvalidRequestBody
}
