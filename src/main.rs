extern crate core;

mod sorting;
mod grouping;
mod exif;
mod r#match;

use image::{GenericImage, GenericImageView};
use std::path::Path;
use crate::exif::extractor::extract;
use crate::grouping::grouper::group;
use crate::sorting::sorter::{sort, SortAlgorithm};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image_dir = Path::new("../writeups/SANS Challenge 2024 - Act I/shreds/slices").to_str().unwrap();

    //sort(image_dir, SortAlgorithm::ByUuidLatestPart);
    //group(image_dir)?;
    //extract(image_dir);

    Ok(())
}