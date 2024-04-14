use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("this function is not implemented for the current platform")]
    NotImplemented,
    #[error("failed while trying to write to a file")]
    WriteFailed,
    #[error("failed while trying to unhook a callback, make sure the id is correct")]
    UnhookFailed,
    #[error("the pointer is not on the same screen as the specified window")]
    X11PointerWindowMismatch,
    #[error("failed to send input, the input was already blocked by another thread")]
    InputIsBlocked,
    #[error("CoreGraphics: failed to create mouse event")]
    CGCouldNotCreateEvent,
    #[error("permission denied for this operation, plese try as super user")]
    PermissionDenied,
    #[error("Error: `{0}`")]
    CustomError(String),
}
