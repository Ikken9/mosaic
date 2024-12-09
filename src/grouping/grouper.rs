use img_hash::{HasherConfig, ImageHash};
use std::collections::HashMap;
use std::path::Path;
use img_hash::image::DynamicImage;
use walkdir::WalkDir;

// Function to compute perceptual hash for an image
fn compute_image_hash(image_path: &Path) -> Result<ImageHash, Box<dyn std::error::Error>> {
    let hasher = HasherConfig::new().hash_size(16, 16).to_hasher();
    let img = image::open(image_path)?;
    let hash = hasher.hash_image(&convert_to_rgb(&img));
    Ok(hash)
}

fn convert_to_rgb(img: &DynamicImage) -> DynamicImage {
    match img {
        DynamicImage::ImageRgb8(_) | DynamicImage::ImageRgba8(_) => img.clone(),
        _ => DynamicImage::ImageRgb8(img.to_rgb8()),
    }
}

// Compute Hamming distance between two hashes
fn hamming_distance(hash1: u64, hash2: u64) -> u32 {
    (hash1 ^ hash2).count_ones()
}

pub fn group(directory: &str) -> Result<(), Box<dyn std::error::Error>> {
    let similarity_threshold = 10;
    let mut groups: HashMap<ImageHash, Vec<String>> = HashMap::new();

    // Traverse the directory and compute hashes
    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                if extension == "png" || extension == "jpg" || extension == "jpeg" {
                    match compute_image_hash(path) {
                        Ok(hash) => {
                            let mut added_to_group = false;
                            for (group_hash, group) in groups.iter_mut() {
                                if group_hash.dist(&hash) <= similarity_threshold {
                                    group.push(path.to_string_lossy().to_string());
                                    added_to_group = true;
                                    break;
                                }
                            }
                            if !added_to_group {
                                groups.insert(hash, vec![path.to_string_lossy().to_string()]);
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to compute hash for {}: {}", path.display(), e);
                        }
                    }
                }
            }
        }
    }

    // Print grouped results
    println!("Grouped Images:");
    for (group_hash, images) in groups.iter() {
        println!("Group Hash: {:?}", group_hash);
        for image in images {
            println!("  - {}", image);
        }
    }

    Ok(())
}