use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EimageError(#[from] eimage_core::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
    #[error(transparent)]
    StdSystemTimeError(#[from] std::time::SystemTimeError),

    #[error("file extension `{0}` is invalid")]
    InvalidFileExtension(String),
    #[error("file extension is invalid")]
    NoFileExtension(),
}
