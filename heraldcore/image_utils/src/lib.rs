use image::{self, FilterType, ImageFormat};
use platform_dirs::pictures_dir;
use std::path::Path;

pub use image::image_dimensions;
pub use image::ImageError;

pub struct Dims {
    pub width: u32,
    pub height: u32,
}

impl From<(u32, u32)> for Dims {
    fn from((width, height): (u32, u32)) -> Self {
        Self { width, height }
    }
}

impl From<Dims> for json::JsonValue {
    fn from(Dims { width, height }: Dims) -> Self {
        json::object! {
            "width" => width,
            "height" => height
        }
    }
}

/// Specifies a path to an image along with the intended cropping behavior
pub struct ProfilePicture {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    pub path: String,
}

impl ProfilePicture {
    const SIZE: u32 = 300;

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

    let (src_width, src_height) = image_dimensions(&path)?;
    let Dims {
        width: scaled_width,
        ..
    } = image_scaling_dims(src_width, src_height, ProfilePicture::SIZE);

    let t = |a| ((a as f32) * (src_width as f32) / (scaled_width as f32)) as u32;

    let x = t(x);
    let y = t(y);
    let width = t(width);
    let height = t(height);

    let image_path = image_path();

    image::open(path)?
        .crop(x, y, width, height)
        .resize_exact(
            ProfilePicture::SIZE,
            ProfilePicture::SIZE,
            FilterType::Nearest,
        )
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
        .crop(0, 0, ProfilePicture::SIZE, ProfilePicture::SIZE)
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
        .crop(0, 0, ProfilePicture::SIZE, ProfilePicture::SIZE)
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

/// Given image dimensions and a constant, scales the smaller dimension down
/// and makes the larger dimension equal to the constant
pub fn image_scaling<P>(
    source: P,
    scale: u32,
) -> Result<Dims, image::ImageError>
where
    P: AsRef<Path>,
{
    let (width, height) = image_dimensions(source)?;
    Ok(image_scaling_dims(width, height, scale))
}

fn image_scaling_dims(
    width: u32,
    height: u32,
    scale: u32,
) -> Dims {
    let (width, height, scale) = (width as f32, height as f32, scale as f32);

    let (width, height) = if width > height {
        (scale * width / height, scale)
    } else {
        (scale, scale * height / width)
    };

    Dims {
        width: width.ceil().max(scale) as u32,
        height: height.ceil().max(scale) as u32,
    }
}

pub fn image_scaling_reverse<P>(
    source: P,
    scale: u32,
) -> Result<Dims, image::ImageError>
where
    P: AsRef<Path>,
{
    let (width, height) = image_dimensions(source)?;
    let (width, height, scale) = (width as f32, height as f32, scale as f32);

    let (width, height) = if width > height {
        (scale, scale * height / width)
    } else {
        (scale * width / height, scale)
    };

    Ok(Dims {
        width: width.ceil().max(scale) as u32,
        height: height.ceil().max(scale) as u32,
    })
}
