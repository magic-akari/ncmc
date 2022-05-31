use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Type {
    Png,
    Jpeg,
    Gif,
    Bmp,
    Webp,
    Unknown,
}

#[derive(Clone)]
pub struct Image(Type, Vec<u8>);

impl Image {
    pub fn ext(&self) -> String {
        match &self.0 {
            Type::Png => "png".into(),
            Type::Jpeg => "jpeg".into(),
            Type::Gif => "gif".into(),
            Type::Bmp => "bmp".into(),
            Type::Webp => "webp".into(),
            Type::Unknown => "image".into(),
        }
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.1
    }

    pub fn into_data(self) -> Vec<u8> {
        self.1
    }
}

impl From<Vec<u8>> for Image {
    fn from(value: Vec<u8>) -> Self {
        match (&value[..4], &value[4..8], &value[8..12]) {
            (b"\x89PNG", [0x0D, 0x0A, 0x1A, 0x0A], _) => Image(Type::Png, value),
            ([0xFF, 0xD8, 0xFF, 0xE0 | 0xE1 | 0xE2 | 0xE3 | 0xE8], ..) => Image(Type::Jpeg, value),
            (b"RIFF", _, b"WEBP") => Image(Type::Webp, value),
            (b"GIF8", ..) => Image(Type::Gif, value),
            ([b'B', b'M', ..], ..) => Image(Type::Bmp, value),
            _ => Image(Type::Unknown, value),
        }
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Image")
            .field("type", &self.0)
            .field("size", &self.1.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::Image;

    #[test]
    fn test_image() {
        let mut data = vec![0; 32];
        data[..4].copy_from_slice(b"GIF8");
        let image = Image::from(data);
        assert_eq!(image.ext(), "gif");

        let mut data = vec![0; 32];
        data[..4].copy_from_slice(&[0xFF, 0xD8, 0xFF, 0xE0]);
        let image = Image::from(data);
        assert_eq!(image.ext(), "jpeg");
    }
}
