use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// Generic error type for any errors happening in the virtualfs crate.
#[derive(Error, Debug)]
pub enum Error {
    /// Error coming from IO operations.
    #[error("an IO error happened: {0:?}")]
    Io(#[from] IoError),

    #[error("Custom error: {0}")]
    Custom(#[from] Box<dyn std::error::Error + Send + Sync>),

    /// Represents any error that did not have an actual type attached to it.
    /// For example, custom implementations can use this type to provide
    /// information.
    #[error(r#"Custom error string: "{0}""#)]
    String(String),
}

impl Error {
    pub fn custom<E: std::error::Error + Send + Sync + 'static>(err: E) -> Self {
        Self::Custom(Box::new(err))
    }

    pub fn get_io(&self) -> Option<&IoError> {
        match self {
            Error::Io(x) => Some(x),
            _ => None,
        }
    }

    pub fn get_custom(&self) -> Option<&dyn std::error::Error> {
        match self {
            Error::Custom(x) => Some(x.as_ref()),
            _ => None,
        }
    }

    pub fn get_string(&self) -> Option<&String> {
        match self {
            Error::String(ref x) => Some(x),
            _ => None,
        }
    }
}

#[cfg(not(feature = "no_std"))]
impl Into<std::io::Error> for Error {
    fn into(self) -> std::io::Error {
        match self {
            Error::Io(io) => io.into(),
            x => std::io::Error::new(std::io::ErrorKind::Other, Box::new(x)),
        }
    }
}

/// A symmetrical error enum to [`std::io::ErrorKind`]. Because we need to support
/// [`no_std`], we cannot use [`std::io::Error`] directly. We thus employ this error
/// enum, and have a From type to convert from and into a regular [`std::io::Error`]
/// when available.
#[derive(Clone, Copy, Debug, Error, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum IoError {
    #[error("An OS error occured, code {0}")]
    Os(i32),

    #[error("An entity was not found, often a file.")]
    NotFound,

    #[error("The operation lacked the necessary privileges to complete.")]
    PermissionDenied,

    #[error("The connection was refused by the remote server.")]
    ConnectionRefused,

    #[error("The connection was reset by the remote server.")]
    ConnectionReset,

    #[error("The connection was aborted (terminated) by the remote server.")]
    ConnectionAborted,

    #[error("The network operation failed because it was not connected yet.")]
    NotConnected,

    #[error(
        "A socket address could not be bound because the address is already in use elsewhere."
    )]
    AddrInUse,

    #[error("A nonexistent interface was requested or the requested address was not local.")]
    AddrNotAvailable,

    #[error("The operation failed because a pipe was closed.")]
    BrokenPipe,

    #[error("An entity already exists, often a file.")]
    AlreadyExists,

    #[error("The operation needs to block to complete, but the blocking operation was requested to not occur.")]
    WouldBlock,

    #[error("A parameter was incorrect.")]
    InvalidInput,

    #[error("Data not valid for the operation were encountered.")]
    InvalidData,

    #[error("The I/O operation's timeout expired, causing it to be canceled.")]
    TimedOut,

    #[error("An error returned when an operation could not be completed because a call to write returned Ok(0).")]
    WriteZero,

    #[error("This operation was interrupted.")]
    Interrupted,

    #[error("Any I/O error not part of this list.")]
    Other,

    #[error(r#"An error returned when an operation could not be completed because an"end of file" was reached prematurely."#)]
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

#[cfg(not(feature = "no_std"))]
impl Into<std::io::Error> for IoError {
    fn into(self) -> std::io::Error {
        match self {
            IoError::Os(code) => std::io::Error::from_raw_os_error(code),

            IoError::NotFound => std::io::Error::from(std::io::ErrorKind::NotFound),
            IoError::PermissionDenied => std::io::Error::from(std::io::ErrorKind::PermissionDenied),
            IoError::ConnectionRefused => {
                std::io::Error::from(std::io::ErrorKind::ConnectionRefused)
            }
            IoError::ConnectionReset => std::io::Error::from(std::io::ErrorKind::ConnectionReset),
            IoError::ConnectionAborted => {
                std::io::Error::from(std::io::ErrorKind::ConnectionAborted)
            }
            IoError::NotConnected => std::io::Error::from(std::io::ErrorKind::NotConnected),
            IoError::AddrInUse => std::io::Error::from(std::io::ErrorKind::AddrInUse),
            IoError::AddrNotAvailable => std::io::Error::from(std::io::ErrorKind::AddrNotAvailable),
            IoError::BrokenPipe => std::io::Error::from(std::io::ErrorKind::BrokenPipe),
            IoError::AlreadyExists => std::io::Error::from(std::io::ErrorKind::AlreadyExists),
            IoError::WouldBlock => std::io::Error::from(std::io::ErrorKind::WouldBlock),
            IoError::InvalidInput => std::io::Error::from(std::io::ErrorKind::InvalidInput),
            IoError::InvalidData => std::io::Error::from(std::io::ErrorKind::InvalidData),
            IoError::TimedOut => std::io::Error::from(std::io::ErrorKind::TimedOut),
            IoError::WriteZero => std::io::Error::from(std::io::ErrorKind::WriteZero),
            IoError::Interrupted => std::io::Error::from(std::io::ErrorKind::Interrupted),
            IoError::Other => std::io::Error::from(std::io::ErrorKind::Other),
            IoError::UnexpectedEof => std::io::Error::from(std::io::ErrorKind::UnexpectedEof),
        }
    }
}
