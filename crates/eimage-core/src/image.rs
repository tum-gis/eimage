use chrono::{DateTime, Utc};
use image::{ImageBuffer, Rgb};

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    buffer: ImageBuffer<Rgb<u8>, Vec<u8>>,
    timestamp: DateTime<Utc>,
}

impl Image {
    pub fn new(buffer: ImageBuffer<Rgb<u8>, Vec<u8>>, timestamp: DateTime<Utc>) -> Self {
        Self { buffer, timestamp }
    }

    pub fn get_buffer(&self) -> &ImageBuffer<Rgb<u8>, Vec<u8>> {
        &self.buffer
    }
}
