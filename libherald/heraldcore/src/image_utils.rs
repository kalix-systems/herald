use crate::errors::HErr;
use image::{self, FilterType, ImageFormat};
use lazy_static::*;
use std::path::{Path, PathBuf};

const IMAGE_SIZE: u32 = 300;
const PROFILE_PICTURE: &str = "profile_pictures";

lazy_static! {
    static ref IMAGE_PATH: PathBuf = PathBuf::from(PROFILE_PICTURE);
}

/// Determines path of profile picture for user id.
pub fn profile_picture_path(id: &str) -> PathBuf {
    let mut image_path = IMAGE_PATH.clone();
    image_path.push(format!("{:?}_{}.png", std::time::Instant::now(), id));
    image_path
}

/// Given a path to an existing picture (`source`), generates a thumbnail and moves the picture to
/// herald's storage.
pub fn save_profile_picture<P>(id: &str, source: P) -> Result<PathBuf, HErr>
where
    P: AsRef<Path>,
{
    std::fs::create_dir_all(PROFILE_PICTURE)?;

    let image_path = profile_picture_path(id);

    image::open(source)?
        .resize_exact(IMAGE_SIZE, IMAGE_SIZE, FilterType::Nearest)
        .save_with_format(&image_path, ImageFormat::PNG)?;
    Ok(image_path)
}

pub fn delete_profile_picture(id: &str) -> Result<(), HErr> {
    let image_path = profile_picture_path(id);

    Ok(std::fs::remove_file(image_path)?)
}
