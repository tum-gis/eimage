use crate::Error;
use eimage_core::ImageCollection;
use std::io::{Cursor, Write};
use std::path::PathBuf;
use tar::Builder;

pub const DEFAULT_IMAGES_PATH: &str = "images";
pub const DEFAULT_ROS_TOPIC_SEPARATOR: &str = "/";

pub fn write_eimage_format<W: Write>(
    writer: W,
    image_collection: ImageCollection,
    _compression_level: Option<i32>,
) -> Result<(), Error> {
    let current_timestamp_sec = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    let mut archive_builder = Builder::new(writer);

    // image collection
    let image_directory_path = PathBuf::from(DEFAULT_IMAGES_PATH);
    archive_builder.append_dir(&image_directory_path, ".")?;

    for current_frame_id in image_collection.get_frame_ids() {
        let current_image_series = image_collection.get_image_series(&current_frame_id)?;
        let current_images = current_image_series.get_images();
        let sanitized_directory_name = current_frame_id
            .to_string()
            .trim_start_matches(DEFAULT_ROS_TOPIC_SEPARATOR)
            .replace(DEFAULT_ROS_TOPIC_SEPARATOR, "___");
        let frame_directory_path = image_directory_path.join(sanitized_directory_name);

        for (current_index, current_image) in current_images.iter().enumerate() {
            let buffer = current_image.get_buffer();
            let file_path = frame_directory_path.join(current_index.to_string() + ".png");

            let mut image_data_buffer: Vec<u8> = Vec::new();
            buffer.write_to(
                &mut Cursor::new(&mut image_data_buffer),
                image::ImageFormat::Png,
            )?;

            archive_builder.append_data(
                &mut create_archive_header(image_data_buffer.len(), current_timestamp_sec),
                file_path,
                Cursor::new(image_data_buffer),
            )?;
        }

        // current_image_series.
        archive_builder.append_dir(frame_directory_path, ".")?;
    }

    archive_builder.finish()?;
    Ok(())
}

fn create_archive_header(size: usize, timestamp_sec: u64) -> tar::Header {
    let mut header = tar::Header::new_gnu();
    header.set_size(size as u64);
    header.set_mode(0o664);
    header.set_mtime(timestamp_sec);
    header.set_cksum();

    header
}
