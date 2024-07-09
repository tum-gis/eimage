use crate::error::Error;
use crate::error::Error::ContainsNoImages;
use crate::image::Image;

#[derive(Debug, Clone, PartialEq)]
pub struct ImageSeries {
    images: Vec<Image>,
}

impl ImageSeries {
    pub fn new(images: Vec<Image>) -> Result<Self, Error> {
        if images.is_empty() {
            return Err(ContainsNoImages);
        }

        Ok(Self { images })
    }

    pub fn get_images(&self) -> &Vec<Image> {
        &self.images
    }

    pub fn len(&self) -> usize {
        self.images.len()
    }
}
