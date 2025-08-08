use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EimageError(#[from] eimage::Error),
    #[error(transparent)]
    EimageIoError(#[from] eimage::io::Error),

    #[error(transparent)]
    EcoordError(#[from] ecoord::Error),

    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
    #[error(transparent)]
    ImageResult(#[from] image::ImageError),
}
