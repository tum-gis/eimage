//! `eimage` is a library for processing image collections in 3D space.
//!
//!
//! # Overview
//!
//!
//! # Data structure
//!
//! For serializing an image collection, this data structure is used:
//!
//! - `image_collection_name.tar` (uncompressed as [tarball](https://en.wikipedia.org/wiki/Tar_(computing))) or `image_collection_name.eimage` (compressed)
//!     - directory `images`:
//!         - directory `frame_id`:
//!             - `1.png`
//!             - `2.png`
//!             - ...
//!             - `n.png`
//!     - `info.json` (uncompressed) or `info.json.zst` (compressed)
//!         - mandatory fields:
//!         - optional fields:
//!     - `ecoord.json` (uncompressed) or `ecoord.json.zst` (compressed)
//!         - contains a transformation tree with validity durations
//!         - information: srid
//!         - purpose: translate and rotate the point cloud without reading/writing the point data

pub use eimage_core::{Error, Image, ImageCollection, ImageSeries};

pub use eimage_io as io;
