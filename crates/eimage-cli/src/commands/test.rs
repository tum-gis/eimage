use ecoord::{FrameId, ReferenceFrames};
use std::collections::HashMap;

use eimage::{ImageCollection, ImageSeries};
use image::io::Reader as ImageReader;
use image::{ImageBuffer, Rgb};
use std::path::Path;
use tracing::info;

pub fn run(file_path: impl AsRef<Path>, output_file_path: impl AsRef<Path>) {
    info!("Path: {:?}", file_path.as_ref().to_owned());
    let individual_image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageReader::open(file_path)
        .unwrap()
        .decode()
        .unwrap()
        .into_rgb8();
    let individual_image: eimage::Image =
        eimage::Image::new(individual_image, chrono::offset::Utc::now());
    // let img2 = ImageReader::new(Cursor::new(bytes)).with_guessed_format()?.decode()?;

    let reference_frames = ReferenceFrames::new(
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
    )
    .unwrap();

    let image_series = ImageSeries::new(vec![individual_image]).unwrap();
    let mut image_series_map: HashMap<FrameId, ImageSeries> = HashMap::new();
    image_series_map.insert(FrameId::from("test"), image_series);
    let image_collection = ImageCollection::new(image_series_map, reference_frames).unwrap();

    eimage::io::EimageWriter::from_path(output_file_path)
        .unwrap()
        .finish(image_collection)
        .unwrap();

    // info!("test: {:?}", individual_image.dimensions());
}
