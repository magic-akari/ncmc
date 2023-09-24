use crate::ncm_rc4::NcmRc4;
use anyhow::Result;
use std::{
    fmt::{Debug, Display},
    io::{Chain, Cursor, Read},
    iter::Cycle,
};

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Flac,
    Mp3,
    M4a,
    Ogg,
    Unknown,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ext = match &self {
            Type::Flac => "flac",
            Type::Mp3 => "mp3",
            Type::M4a => "m4a",
            Type::Ogg => "ogg",
            Type::Unknown => "audio",
        };

        write!(f, "{}", ext)
    }
}

impl From<[u8; 12]> for Type {
    fn from(value: [u8; 12]) -> Self {
        match &value[..4] {
            b"fLaC" => Type::Flac,
            b"OggS" => Type::Ogg,
            [0xFF, 0xFB, ..] => Type::Mp3,
            [b'I', b'D', b'3', ..] => Type::Mp3,
            _ => {
                if &value[4..12] == b"ftypM4A " {
                    Type::M4a
                } else {
                    Type::Unknown
                }
            }
        }
    }
}

type Rc4Iter = Cycle<std::array::IntoIter<u8, 256_usize>>;

pub struct Audio<R>
where
    R: Read,
{
    r#type: Type,
    rc4_iter: Rc4Iter,
    reader: Chain<Cursor<[u8; 12]>, R>,
}

impl<R> Audio<R>
where
    R: Read,
{
    pub fn try_new(mut input: R, key: &[u8]) -> Result<Self> {
        let rc4_iter = NcmRc4::new(key).into_iter().cycle();

        let mut buf = [0; 12];
        input.read_exact(&mut buf)?;

        let reader = Cursor::new(buf).chain(input);

        let r#type = {
            let mut tmp_iter = rc4_iter.clone();
            Self::decrypt(&mut tmp_iter, &mut buf);
            buf.into()
        };

        Ok(Self { r#type, rc4_iter, reader })
    }

    pub fn r#type(&self) -> Type {
        self.r#type
    }

    pub fn ext(&self) -> String {
        self.r#type.to_string()
    }

    fn decrypt(rc4_iter: &mut Rc4Iter, buf: &mut [u8]) {
        buf.iter_mut().zip(rc4_iter).for_each(|(byte, x)| *byte ^= x);
    }
}

impl<R> Read for Audio<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let size = self.reader.read(buf)?;
        Self::decrypt(&mut self.rc4_iter, &mut buf[..size]);
        Ok(size)
    }
}

impl<R> Debug for Audio<R>
where
    R: Read,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Audio").field(&format!("{}", self.r#type)).finish()
    }
}
