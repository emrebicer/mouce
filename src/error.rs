use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotImplemented,
    WriteFailed,
    X11PointerWindowMismatch,
    InputIsBlocked,
    CGCouldNotCreateEvent,
    CustomError(&'static str),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_message = match self {
            Error::NotImplemented => "this function is not implemented for the current platform",
            Error::WriteFailed => "failed while trying to write to a file",
            Error::X11PointerWindowMismatch => {
                "the pointer is not on the same screen as the specified window"
            }
            Error::InputIsBlocked => {
                "failed to send input, the input was already blocked by another thread"
            }
            Error::CGCouldNotCreateEvent => "CoreGraphics: failed to create mouse event",
            Error::CustomError(err_description) => err_description,
        };

        write!(f, "{}", err_message)
    }
}
