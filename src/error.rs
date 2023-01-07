use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotImplemented,
    WriteFailed,
    UnhookFailed,
    X11PointerWindowMismatch,
    InputIsBlocked,
    CGCouldNotCreateEvent,
    PermissionDenied,
    CustomError(&'static str),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_message = match self {
            Error::NotImplemented => "this function is not implemented for the current platform",
            Error::WriteFailed => "failed while trying to write to a file",
            Error::UnhookFailed => {
                "failed while trying to unhook a callback, make sure the id is correct"
            }
            Error::X11PointerWindowMismatch => {
                "the pointer is not on the same screen as the specified window"
            }
            Error::InputIsBlocked => {
                "failed to send input, the input was already blocked by another thread"
            }
            Error::CGCouldNotCreateEvent => "CoreGraphics: failed to create mouse event",
            Error::PermissionDenied => {
                "permission denied for this operation, plese try as super user"
            }
            Error::CustomError(err_description) => err_description,
        };

        write!(f, "{}", err_message)
    }
}
