use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use image::imageops::FilterType;

fn main() {
    // configuration
    let input_path = "./input_images";
    let output_path = "./output_images";

    let args: Vec<String> = env::args().collect();
    let path = PathBuf::from(input_path);

    match count_images(&path) {
        Ok(count) => println!("Found {} image(s) (png, jpg, jpeg)", count),
        Err(e) => eprintln!("Error traversing {}: {}", path.display(), e),
    }

    if args.len() > 1 && args[1] == "--resize" {
        match resize_images(&path, Path::new(output_path), 800, 600) {
            Ok(_) => println!("Images resized to {}", output_path),
            Err(e) => eprintln!("Failed to resize images: {}", e),
        }
    }
}

fn count_images(dir: &Path) -> std::io::Result<u64> {
    let mut count = 0;

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                count += count_images(&path)?;
            } else if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    let ext = ext.to_lowercase();
                    if ext == "png" || ext == "jpg" || ext == "jpeg" {
                        count += 1;
                    }
                }
            }
        }
    }

    Ok(count)
}

fn resize_images(
    input: &Path,
    output: &Path,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    if !output.exists() {
        fs::create_dir_all(output)?;
    }

    if input.is_dir() {
        for entry in fs::read_dir(input)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let sub_output = output.join(entry.file_name());
                resize_images(&path, &sub_output, width, height)?;
            } else if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    let ext = ext.to_lowercase();
                    if ext == "png" || ext == "jpg" || ext == "jpeg" {
                        let img = image::open(&path)?;
                        let resized = img.resize(width, height, FilterType::Lanczos3);
                        let out_path = output.join(path.file_name().unwrap());
                        resized.save(out_path)?;
                    }
                }
            }
        }
    }

    Ok(())
}