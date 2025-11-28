use crate::error::Error;
use crate::error::Error::ContainsNoImageSeries;
use crate::image_series::ImageSeries;
use ecoord::{FrameId, TransformTree};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct ImageCollection {
    pub(crate) image_series: HashMap<FrameId, ImageSeries>,
    pub(crate) transform_tree: TransformTree,
}

impl ImageCollection {
    pub fn new(
        image_series: HashMap<FrameId, ImageSeries>,
        transform_tree: TransformTree,
    ) -> Result<Self, Error> {
        if image_series.is_empty() {
            return Err(ContainsNoImageSeries);
        }

        let image_collection = Self {
            image_series,
            transform_tree,
        };
        Ok(image_collection)
    }

    pub fn get_frame_ids(&self) -> HashSet<FrameId> {
        HashSet::from_iter(self.image_series.keys().cloned())
    }

    pub fn get_image_series(&self, frame_id: &FrameId) -> Result<&ImageSeries, Error> {
        self.image_series.get(frame_id).ok_or(ContainsNoImageSeries)
    }

    pub fn total_image_count(&self) -> usize {
        self.image_series.values().map(|x| x.len()).sum()
    }
}
