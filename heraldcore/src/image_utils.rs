use crate::errors::HErr;
use image::{self, FilterType, ImageFormat};
use platform_dirs::pictures_dir;
use std::path::{Path, PathBuf};

const IMAGE_SIZE: u32 = 300;

/// Given a path to an existing picture (`source`), generates a thumbnail and moves the picture to
/// herald's storage.
pub(crate) fn update_picture<P>(
    source: P,
    old_path: Option<P>,
) -> Result<PathBuf, HErr>
where
    P: AsRef<Path>,
{
    std::fs::create_dir_all(pictures_dir())?;

    if let Some(old_path) = old_path {
        std::fs::remove_file(old_path)?;
    }

    let image_path = image_path();

    image::open(source)?
        .resize_exact(IMAGE_SIZE, IMAGE_SIZE, FilterType::Nearest)
        .save_with_format(&image_path, ImageFormat::PNG)?;

    Ok(image_path)
}

pub(crate) fn image_path() -> PathBuf {
    let rid = kcl::random::UQ::gen_new();
    let text = hex::encode(rid.as_ref());

    let mut image_path = pictures_dir().join(text);
    image_path.set_extension("png");

    image_path
}

/// Returns image dimensions
pub fn image_dimensions<P>(source: P) -> Result<(u32, u32), HErr>
where
    P: AsRef<Path>,
{
    Ok(image::image_dimensions(source)?)
}
