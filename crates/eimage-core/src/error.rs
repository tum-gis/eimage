use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("path is not a directory")]
    ContainsNoImages,

    #[error("path is not a directory")]
    ContainsNoImageSeries,
}
