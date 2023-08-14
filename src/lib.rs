#[derive(Default)]
pub struct ImageBuffer {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

impl ImageBuffer {
    pub fn new(width: usize, height: usize) -> ImageBuffer {
        ImageBuffer {
            width: width,
            height: height,
            data: vec![0; width * height * 4],
        }
    }
}

#[derive(Debug)]
pub struct Error(String);
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}
impl std::error::Error for Error {}

pub mod jpeg;
pub mod png;

/// Guess filetype and decode
pub fn decode(src: &[u8]) -> Result<ImageBuffer, Error> {
    if src.len() >= 2 && src[0..2] == [0xFF, 0xD8] {
        return jpeg::decode(src);
    } else if src.len() >= 8 && src[0..8] == [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] {
        return png::decode(src);
    }

    return Err(Error("Unsupported format".to_string()));
}
