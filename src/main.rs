use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, RgbImage};
use std::fs;
use std::path::PathBuf;
use std::error::Error;
use std::io;

fn load_images(folder: &str) -> Result<Vec<DynamicImage>, io::Error> {
    let mut files: Vec<PathBuf> = fs::read_dir(folder)?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| {
            if let Some(ext) = p.extension() {
                let ext_str = ext.to_str().unwrap().to_lowercase();
                ext_str == "png" || ext_str == "jpg" || ext_str == "jpeg"
            } else {
                false
            }
        })
        .collect();
    files.sort();

    let mut images = Vec::new();
    for file in files {
        let img = image::open(&file)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to open {:?}: {}", file, e)))?;
        let img_rgb = img.to_rgb8();
        images.push(DynamicImage::ImageRgb8(img_rgb));
    }
    Ok(images)
}

fn calculate_difference(slice1: &DynamicImage, slice2: &DynamicImage) -> u64 {
    let (w1, h1) = slice1.dimensions();
    let (w2, h2) = slice2.dimensions();
    let h = std::cmp::min(h1, h2);

    let mut sum = 0u64;
    for y in 0..h {
        print!("xd");
        let p1 = slice1.get_pixel(w1 - 1, y);
        let p2 = slice2.get_pixel(0, y);

        let dr = (p1[0] as i32 - p2[0] as i32).pow(2) as u64;
        let dg = (p1[1] as i32 - p2[1] as i32).pow(2) as u64;
        let db = (p1[2] as i32 - p2[2] as i32).pow(2) as u64;
        sum += dr + dg + db;
    }
    sum
}

fn find_best_match(mut slices: Vec<DynamicImage>) -> Vec<DynamicImage> {
    if slices.is_empty() {
        return slices;
    }
    let mut matched_slices = Vec::new();
    matched_slices.push(slices.remove(0));

    while !slices.is_empty() {
        let last_slice = matched_slices.last().unwrap();
        let differences: Vec<u64> = slices
            .iter()
            .map(|s| calculate_difference(last_slice, s))
            .collect();

        let (best_match_index, _) = differences
            .iter()
            .enumerate()
            .min_by_key(|&(_, &d)| d)
            .unwrap();

        matched_slices.push(slices.remove(best_match_index));
    }

    matched_slices
}

fn save_image(images: &[DynamicImage], output_path: &str) -> Result<(), Box<dyn Error>> {
    let mut total_width = 0;
    let mut max_height = 0;
    for img in images {
        let (w, h) = img.dimensions();
        total_width += w;
        if h > max_height {
            max_height = h;
        }
    }

    let mut new_img: RgbImage = ImageBuffer::new(total_width, max_height);
    let mut x_offset = 0;
    for img in images {
        let (w, h) = img.dimensions();
        let rgb = img.to_rgb8();
        for y in 0..h {
            for x in 0..w {
                let p = rgb.get_pixel(x, y);
                new_img.put_pixel(x_offset + x, y, Rgb([p[0], p[1], p[2]]));
            }
        }
        x_offset += w;
    }

    new_img.save(output_path)?;
    Ok(())
}

fn run(image_dir: &str) -> Result<(), Box<dyn Error>> {
    let slices = load_images(image_dir)?;
    if slices.is_empty() {
        return Err("No images found in the directory.".into());
    }
    let matched_slices = find_best_match(slices);
    let output_path = "./assembled_image.png";
    save_image(&matched_slices, output_path)?;
    println!("Image assembled and saved to {}", output_path);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Just set the directory here. This should be all you need to do.
    let image_dir = std::path::Path::new("../writeups/SANS Challenge 2024 - Act I/shreds/slices")
        .to_str()
        .unwrap();
    run(image_dir)?;
    Ok(())
}
