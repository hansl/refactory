use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// Generic error type for any errors happening in the virtualfs crate.
#[derive(Error, Debug)]
pub enum Error {
    /// Error coming from IO operations.
    #[error("an IO error happened: {0:?}")]
    Io(IoError),

    /// Represents any error that did not have an actual type attached to it.
    /// For example, custom implementations can use this type to provide
    /// information.
    #[error(r#"Custom error: "{0}""#)]
    Custom(String),
}

/// A symmetrical error enum to [`std::io::ErrorKind`]. Because we need to support
/// [`no_std`], we cannot use [`std::io::Error`] directly. We thus employ this error
/// enum, and have a From type to convert from and into a regular [`std::io::Error`]
/// when available.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IoError {
    Os(i32),

    /// An entity was not found, often a file.
    NotFound,

    /// The operation lacked the necessary privileges to complete.
    PermissionDenied,

    /// The connection was refused by the remote server.
    ConnectionRefused,

    /// The connection was reset by the remote server.
    ConnectionReset,

    /// The connection was aborted (terminated) by the remote server.
    ConnectionAborted,

    /// The network operation failed because it was not connected yet.
    NotConnected,

    /// A socket address could not be bound because the address is already in
    /// use elsewhere.
    AddrInUse,

    /// A nonexistent interface was requested or the requested address was not
    /// local.
    AddrNotAvailable,

    /// The operation failed because a pipe was closed.
    BrokenPipe,

    /// An entity already exists, often a file.
    AlreadyExists,

    /// The operation needs to block to complete, but the blocking operation was
    /// requested to not occur.
    WouldBlock,

    /// A parameter was incorrect.
    InvalidInput,

    /// Data not valid for the operation were encountered.
    ///
    /// Unlike [`InvalidInput`], this typically means that the operation
    /// parameters were valid, however the error was caused by malformed
    /// input data.
    ///
    /// For example, a function that reads a file into a string will error with
    /// `InvalidData` if the file's contents are not valid UTF-8.
    ///
    /// [`InvalidInput`]: #variant.InvalidInput
    InvalidData,

    /// The I/O operation's timeout expired, causing it to be canceled.
    TimedOut,

    /// An error returned when an operation could not be completed because a
    /// call to [`write`] returned [`Ok(0)`].
    ///
    /// This typically means that an operation could only succeed if it wrote a
    /// particular number of bytes but only a smaller number of bytes could be
    /// written.
    ///
    /// [`write`]: ../../std/io/trait.Write.html#tymethod.write
    /// [`Ok(0)`]: ../../std/io/type.Result.html
    WriteZero,

    /// This operation was interrupted.
    ///
    /// Interrupted operations can typically be retried.
    Interrupted,

    /// Any I/O error not part of this list.
    Other,

    /// An error returned when an operation could not be completed because an
    /// "end of file" was reached prematurely.
    ///
    /// This typically means that an operation could only succeed if it read a
    /// particular number of bytes but only a smaller number of bytes could be
    /// read.
    UnexpectedEof,
}

#[cfg(not(feature = "no_std"))]
impl From<std::io::Error> for IoError {
    fn from(err: std::io::Error) -> Self {
        if let Some(os_code) = err.raw_os_error() {
            IoError::Os(os_code)
        } else {
            match err.kind() {
                std::io::ErrorKind::NotFound => IoError::NotFound,
                std::io::ErrorKind::PermissionDenied => IoError::PermissionDenied,
                std::io::ErrorKind::ConnectionRefused => IoError::ConnectionRefused,
                std::io::ErrorKind::ConnectionReset => IoError::ConnectionReset,
                std::io::ErrorKind::ConnectionAborted => IoError::ConnectionAborted,
                std::io::ErrorKind::NotConnected => IoError::NotConnected,
                std::io::ErrorKind::AddrInUse => IoError::AddrInUse,
                std::io::ErrorKind::AddrNotAvailable => IoError::AddrNotAvailable,
                std::io::ErrorKind::BrokenPipe => IoError::BrokenPipe,
                std::io::ErrorKind::AlreadyExists => IoError::AlreadyExists,
                std::io::ErrorKind::WouldBlock => IoError::WouldBlock,
                std::io::ErrorKind::InvalidInput => IoError::InvalidInput,
                std::io::ErrorKind::InvalidData => IoError::InvalidData,
                std::io::ErrorKind::TimedOut => IoError::TimedOut,
                std::io::ErrorKind::WriteZero => IoError::WriteZero,
                std::io::ErrorKind::Interrupted => IoError::Interrupted,
                std::io::ErrorKind::Other => IoError::Other,
                std::io::ErrorKind::UnexpectedEof => IoError::UnexpectedEof,
                x => unreachable!("Unknown std::io::ErrorKind: {:?}", x),
            }
        }
    }
}
