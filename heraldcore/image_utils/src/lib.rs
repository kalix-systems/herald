use image::{self, FilterType, ImageFormat};
use platform_dirs::pictures_dir;
use std::path::Path;

pub use image::ImageError;

const IMAGE_SIZE: u32 = 300;

/// Specifies a path to an image along with the intended cropping behavior
pub struct ProfilePicture {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    pub path: String,
}

impl ProfilePicture {
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
        let path = obj.remove("path")?.as_str()?.to_owned();

        Some(Self {
            x,
            y,
            width,
            height,
            path,
        })
    }

    pub fn autocrop(path: String) -> Self {
        Self {
            x: 0,
            y: 0,
            width: 300,
            height: 300,
            path,
        }
    }
}

/// Given a path to an existing picture (`source`), generates a thumbnail with the provided tag
/// information and moves the picture to herald's storage.
pub fn update_picture(
    ProfilePicture {
        x,
        y,
        width,
        height,
        path,
    }: ProfilePicture
) -> Result<String, image::ImageError> {
    std::fs::create_dir_all(pictures_dir())?;

    let image_path = image_path();

    image::open(path)?
        .crop(x, y, width, height)
        .resize_exact(IMAGE_SIZE, IMAGE_SIZE, FilterType::Nearest)
        .save_with_format(&image_path, ImageFormat::PNG)?;

    Ok(image_path)
}

/// Given a path to an existing picture (`source`), generates a thumbnail and moves the picture to
/// herald's storage.
pub fn update_picture_autocrop<P>(path: P) -> Result<String, image::ImageError>
where
    P: AsRef<Path>,
{
    std::fs::create_dir_all(pictures_dir())?;

    let image_path = image_path();

    image::open(path)?
        .crop(0, 0, IMAGE_SIZE, IMAGE_SIZE)
        .save_with_format(&image_path, ImageFormat::PNG)?;

    Ok(image_path)
}

/// Given a raw image buffer, generates a thumbnail and moves the picture to
/// herald's storage.
pub fn update_picture_buf(buf: &[u8]) -> Result<String, image::ImageError>
where
{
    std::fs::create_dir_all(pictures_dir())?;

    let image_path = image_path();

    image::load_from_memory(buf)?
        .crop(0, 0, IMAGE_SIZE, IMAGE_SIZE)
        .save_with_format(&image_path, ImageFormat::PNG)?;

    Ok(image_path)
}

pub fn image_path() -> String {
    let rid = kcl::random::UQ::gen_new();
    let text = hex::encode(rid.as_ref());

    let mut image_path = pictures_dir().join(text);
    image_path.set_extension("png");

    image_path.into_os_string().to_string_lossy().to_string()
}

/// Returns image dimensions
pub fn image_dimensions<P>(source: P) -> Result<(u32, u32), image::ImageError>
where
    P: AsRef<Path>,
{
    Ok(image::image_dimensions(source)?)
}
