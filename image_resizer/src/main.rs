use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    // configuration
    let path = "./input_images";

    let output_path = "./output_path";

    //todo : keep format, set all jpeg, set all jpg, set all png
    //todo : resize new dimension fixed or proportion of original
    //todo : if not same ratio : stripes horizontal or vertical + black stripes, or blurred image itself
    

    let path = PathBuf::from(path);

    match count_images(&path) {
        Ok(count) => println!("Found {} image(s) (png, jpg, jpeg)", count),
        Err(e) => eprintln!("Error traversing {}: {}", path.display(), e),
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