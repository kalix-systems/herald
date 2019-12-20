use image::{self, FilterType, ImageFormat};
use platform_dirs::pictures_dir;
use std::path::{Path, PathBuf};

pub use image::ImageError;

const IMAGE_SIZE: u32 = 300;

/// Rectangle specifies cropping behavior of an image
pub struct CropRect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl CropRect {
    pub fn from_json_string(j: String) -> Option<Self> {
        let val = json::parse(&j).ok()?;

        let mut obj = match val {
            json::JsonValue::Object(object) => object,
            _ => return None,
        };

        let x = obj.remove("x")?.as_u32()?;
        let y = obj.remove("y")?.as_u32()?;
        let width = obj.remove("width")?.as_u32()?;
        let height = obj.remove("height")?.as_u32()?;

        Some(Self {
            x,
            y,
            width,
            height,
        })
    }
}

/// Given a path to an existing picture (`source`), generates a thumbnail and moves the picture to
/// herald's storage.
pub fn update_picture<P>(
    source: P,
    old_path: Option<P>,
    CropRect {
        x,
        y,
        width,
        height,
    }: CropRect,
) -> Result<PathBuf, image::ImageError>
where
    P: AsRef<Path>,
{
    std::fs::create_dir_all(pictures_dir())?;

    if let Some(old_path) = old_path {
        std::fs::remove_file(old_path)?;
    }

    let image_path = image_path();

    image::open(source)?
        .crop(x, y, width, height)
        .resize_exact(IMAGE_SIZE, IMAGE_SIZE, FilterType::Nearest)
        .save_with_format(&image_path, ImageFormat::PNG)?;

    Ok(image_path)
}

pub fn image_path() -> PathBuf {
    let rid = kcl::random::UQ::gen_new();
    let text = hex::encode(rid.as_ref());

    let mut image_path = pictures_dir().join(text);
    image_path.set_extension("png");

    image_path
}

/// Returns image dimensions
pub fn image_dimensions<P>(source: P) -> Result<(u32, u32), image::ImageError>
where
    P: AsRef<Path>,
{
    Ok(image::image_dimensions(source)?)
}
