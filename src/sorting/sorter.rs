use std::fs;
use std::path::PathBuf;
use image::{GenericImage, RgbImage};

pub enum SortAlgorithm {
    Lexicographical,
    ByUuidFirstPart,
    ByUuidLatestPart,
    ReverseLexicographical,
}

pub fn sort_files(mut files: Vec<PathBuf>, algorithm: SortAlgorithm) -> Vec<PathBuf> {
    match algorithm {
        SortAlgorithm::Lexicographical => {
            files.sort();
        }
        SortAlgorithm::ByUuidFirstPart => {
            files.sort_by(sort_uuid_first_part);
        }
        SortAlgorithm::ByUuidLatestPart => {
            files.sort_by(sort_uuid_latest_part);
        }
        SortAlgorithm::ReverseLexicographical => {
            files.sort_by(|a, b| b.cmp(a)); // Reverse lexicographical order
        }
    }
    files
}

pub fn sort_uuid_first_part(a: &PathBuf, b: &PathBuf) -> std::cmp::Ordering {
    let name_a = a.file_stem().and_then(|n| n.to_str()).unwrap_or("");
    let name_b = b.file_stem().and_then(|n| n.to_str()).unwrap_or("");

    let parts_a: Vec<&str> = name_a.split('-').collect();
    let parts_b: Vec<&str> = name_b.split('-').collect();

    for (pa, pb) in parts_a.iter().zip(parts_b.iter()) {
        match pa.parse::<u64>().into_iter().cmp(pb.parse::<u64>()) {
            std::cmp::Ordering::Equal => continue,
            other => return other,
        }
    }
    name_a.cmp(name_b)
}

pub fn sort_uuid_latest_part(a: &PathBuf, b: &PathBuf) -> std::cmp::Ordering {
    // Extract file stem (name without extension)
    let name_a = a.file_stem().and_then(|n| n.to_str()).unwrap_or("");
    let name_b = b.file_stem().and_then(|n| n.to_str()).unwrap_or("");

    // Split the names by '-' to get UUID parts
    let parts_a: Vec<&str> = name_a.split('-').collect();
    let parts_b: Vec<&str> = name_b.split('-').collect();

    // Get the last part of the UUID for comparison
    let last_a = parts_a.last().unwrap_or(&"0"); // Default to "0" if no parts
    let last_b = parts_b.last().unwrap_or(&"0");

    // Compare the last parts as numeric values
    match last_a.parse::<u64>().into_iter().cmp(last_b.parse::<u64>()) {
        std::cmp::Ordering::Equal => name_a.cmp(name_b), // Fallback to name comparison
        other => other,
    }
}

pub fn sort(image_dir: &str, selected_sort: SortAlgorithm) {
    let image_files: Vec<_> = fs::read_dir(image_dir).unwrap()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("jpg"))
        .collect();

    let sorted_files = sort_files(image_files, selected_sort);

    let image_width = 1;
    let image_height = 1000;

    // Mosaic dimensions
    let mosaic_width = sorted_files.len() * image_width;
    let mosaic_height = image_height;

    // Create a blank canvas
    let mut mosaic = RgbImage::new(mosaic_width as u32, mosaic_height as u32);

    for (index, image_path) in sorted_files.iter().enumerate() {
        let img = image::open(image_path).unwrap();

        // Calculate x-offset for each strip
        let x_offset = (index * image_width) as u32;

        // Copy the image onto the mosaic canvas
        mosaic.copy_from(&img.to_rgb8(), x_offset, 0).unwrap();
    }

    // Save the resulting mosaic
    let output_path = "./mosaic.jpg";
    mosaic.save(output_path).unwrap();

    println!("Mosaic created successfully at {}", output_path);
}