use crate::{errors::HErr, platform_dirs::PICTURES_DIR};
use image::{self, FilterType, ImageFormat};
use std::path::{Path, PathBuf};

const IMAGE_SIZE: u32 = 300;

/// Given a path to an existing picture (`source`), generates a thumbnail and moves the picture to
/// herald's storage.
pub fn update_picture<P>(source: P, old_path: Option<P>) -> Result<PathBuf, HErr>
where
    P: AsRef<Path>,
{
    std::fs::create_dir_all(PICTURES_DIR.as_path())?;

    if let Some(old_path) = old_path {
        std::fs::remove_file(old_path)?;
    }

    let rid = crate::utils::rand_id();
    let text = hex::encode(rid);

    let mut image_path = PICTURES_DIR.join(text);
    image_path.set_extension("png");

    image::open(source)?
        .resize_exact(IMAGE_SIZE, IMAGE_SIZE, FilterType::Nearest)
        .save_with_format(&image_path, ImageFormat::PNG)?;

    Ok(image_path)
}
