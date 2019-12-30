use super::*;

#[derive(Debug)]
pub enum Error {
    Read(std::io::Error, Location),
    Write(std::io::Error, Location),
    StripPrefixError(std::path::StripPrefixError, Location),
    Hash,
    InvalidPathComponent(OsString),
    NonUnicodePath(OsString),
    Image(image_utils::ImageError),
}

impl fmt::Display for Error {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        use Error::*;
        match self {
            Read(e, loc) => write!(
                f,
                "Read error processing attachment at {location}: {error}",
                location = loc,
                error = e
            ),
            Write(e, loc) => write!(
                f,
                "Write error processing attachment at {location}: {error}",
                location = loc,
                error = e,
            ),
            NonUnicodePath(os_str) => write!(
                f,
                "Encountered non-unicode path while converting to Strings, path bytes were: {:x?}",
                os_str
            ),
            StripPrefixError(e, loc) => write!(
                f,
                "Strip prefix error saving attachment at {location}: {error}",
                location = loc,
                error = e,
            ),
            InvalidPathComponent(os_str) => write!(
                f,
                "Encountered invalid filename while creating attachment, path bytes were: {:x?}",
                os_str
            ),
            Hash => write!(f, "Couldn't hash attachment data"),
            Image(e) => write!(f, "Couldn't read image dimensions: {}", e),
        }
    }
}

impl std::error::Error for Error {}
