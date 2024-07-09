use crate::eimage::write_impl::write_eimage_format;
use crate::eimage::{FILE_EXTENSION_EIMAGE_COMPRESSED, FILE_EXTENSION_EIMAGE_UNCOMPRESSED};
use crate::Error;
use crate::Error::{InvalidFileExtension, NoFileExtension};
use eimage_core::ImageCollection;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

pub const DEFAULT_COMPRESSION_LEVEL: i32 = 10;

/// `EimageWriter` sets up a writer for the custom reader data structure.
///
#[derive(Debug, Clone)]
pub struct EimageWriter<W: Write> {
    writer: W,
    compression_level: Option<i32>,
}

impl<W: Write> EimageWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            compression_level: Some(DEFAULT_COMPRESSION_LEVEL),
        }
    }

    pub fn with_compressed(mut self, compressed: bool) -> Self {
        if compressed {
            self.compression_level = Some(DEFAULT_COMPRESSION_LEVEL);
        } else {
            self.compression_level = None;
        }
        self
    }

    pub fn finish(self, image_collection: ImageCollection) -> Result<(), Error> {
        write_eimage_format(self.writer, image_collection, self.compression_level)?;

        Ok(())
    }
}

impl EimageWriter<File> {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let extension = path.as_ref().extension().ok_or(NoFileExtension())?;
        if extension != FILE_EXTENSION_EIMAGE_UNCOMPRESSED
            && extension != FILE_EXTENSION_EIMAGE_COMPRESSED
        {
            return Err(InvalidFileExtension(
                extension.to_str().unwrap_or_default().to_string(),
            ));
        }

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        Ok(Self::new(file))
    }
}
