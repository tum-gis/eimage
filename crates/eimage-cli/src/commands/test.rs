use ecoord::{FrameId, ReferenceFrames};
use std::collections::HashMap;

use crate::error::Error;
use eimage::{ImageCollection, ImageSeries};
use image::ImageReader;
use image::{ImageBuffer, Rgb};
use std::path::Path;
use tracing::info;

pub fn run(file_path: impl AsRef<Path>, output_file_path: impl AsRef<Path>) -> Result<(), Error> {
    info!("Path: {:?}", file_path.as_ref().to_owned());
    let individual_image: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageReader::open(file_path)?.decode()?.into_rgb8();
    let individual_image: eimage::Image =
        eimage::Image::new(individual_image, chrono::offset::Utc::now());
    // let img2 = ImageReader::new(Cursor::new(bytes)).with_guessed_format()?.decode()?;

    let reference_frames = ReferenceFrames::new(
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
    )?;

    let image_series = ImageSeries::new(vec![individual_image])?;
    let mut image_series_map: HashMap<FrameId, ImageSeries> = HashMap::new();
    image_series_map.insert(FrameId::from("test"), image_series);
    let image_collection = ImageCollection::new(image_series_map, reference_frames)?;

    eimage::io::EimageWriter::from_path(output_file_path)?.finish(image_collection)?;

    Ok(())
}
