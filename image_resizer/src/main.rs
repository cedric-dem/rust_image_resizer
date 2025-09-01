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

    println!("==> Start counting");
    match count_images(&path) {
        Ok((jpg, jpeg, png)) => {
            println!("==> Finished counting");
            let total = jpg + jpeg + png;

            if args.len() > 1 && args[1] == "--resize" {
                println!("==> Start resize");
                println!("currently done 0% of images");
                if total > 0 {
                    let mut processed = 0;
                    let mut next_progress = 10;
                    match resize_images(
                        &path,
                        Path::new(output_path),
                        800,
                        600,
                        &mut processed,
                        total,
                        &mut next_progress,
                    ) {
                        Ok(_) => {
                            if next_progress <= 100 {
                                println!("currently done 100% of images");
                            }
                            println!("Images resized to {}", output_path);
                        }
                        Err(e) => eprintln!("Failed to resize images: {}", e),
                    }
                } else {
                    println!("currently done 100% of images");
                }
                println!("==> Finished resize");
            }
        }
        Err(e) => eprintln!("Error traversing {}: {}", path.display(), e),
    }
}

fn count_images(dir: &Path) -> std::io::Result<(u64, u64, u64)> {
    let mut count_jpg = 0;
    let mut count_jpeg = 0;
    let mut count_png = 0;

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let (jpg, jpeg, png) = count_images(&path)?;
                count_jpg += jpg;
                count_jpeg += jpeg;
                count_png += png;
            } else if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    let ext = ext.to_lowercase();
                    if ext == "png" {
                        count_png += 1;
                    } else if ext == "jpg" {
                        count_jpg += 1;
                    } else if ext == "jpeg" {
                        count_jpeg += 1;
                    }
                }
            }
        }
    }
    println!(
        "counted {} jpg, {} jpeg, {} png in path : {}/",
        count_jpg,
        count_jpeg,
        count_png,
        dir.display()
    );

    Ok((count_jpg, count_jpeg, count_png))
}

fn resize_images(
    input: &Path,
    output: &Path,
    width: u32,
    height: u32,
    processed: &mut u64,
    total: u64,
    next_progress: &mut u64,
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
                resize_images(
                    &path,
                    &sub_output,
                    width,
                    height,
                    processed,
                    total,
                    next_progress,
                )?;
            } else if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    let ext = ext.to_lowercase();
                    if ext == "png" || ext == "jpg" || ext == "jpeg" {
                        let img = image::open(&path)?;
                        let resized = img.resize(width, height, FilterType::Lanczos3);
                        let out_path = output.join(path.file_name().unwrap());
                        resized.save(out_path)?;
                        *processed += 1;
                        let progress = *processed * 100 / total;
                        while progress >= *next_progress && *next_progress <= 100 {
                            println!("currently done {}% of images", *next_progress);
                            *next_progress += 10;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
