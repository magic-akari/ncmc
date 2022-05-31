use crate::{
    audio::Audio,
    image::Image,
    key::{decrypt_key, decrypt_meta},
};
use anyhow::{ensure, Ok, Result};
use std::io::Read;

#[derive(Debug)]
pub struct Decoder<R>
where
    R: Read,
{
    pub key: Vec<u8>,
    pub comment: Vec<u8>,
    pub meta: Vec<u8>,
    pub image: Option<Image>,
    pub audio: Audio<R>,
}

impl<R> Decoder<R>
where
    R: Read,
{
    pub fn try_new(mut input: R) -> Result<Self> {
        let mut buffer = [0; 10];
        input.read_exact(&mut buffer)?;

        ensure!(&buffer[..8] == b"CTENFDAM", "CTENFDAM file header mismatch");

        let key = {
            let (mut key, _) = Self::read_frame(&mut input)?;

            let key = decrypt_key(&mut key)?;

            ensure!(&key[..17] == b"neteasecloudmusic", "Invalid ncm key");

            key[17..].to_vec()
        };

        let comment = {
            let (mut comment, _) = Self::read_frame(&mut input)?;
            comment.iter_mut().for_each(|byte| *byte ^= 99);

            ensure!(
                &comment[..22] == b"163 key(Don't modify):",
                "Invalid comment"
            );
            comment
        };

        let meta = {
            let meta = &comment[22..];
            let mut meta = base64::decode(meta)?;

            let meta = decrypt_meta(&mut meta)?;

            ensure!(&meta[..6] == b"music:", "Invalid meta");
            meta[6..].to_vec()
        };

        Self::skip(&mut input, 5)?;

        let image = {
            let offset = Self::read_len(&mut input)?;

            let (image, img_len) = Self::read_frame(&mut input)?;

            if offset > img_len {
                Self::skip(&mut input, (offset - img_len) as usize)?;
            }

            if img_len > 0 {
                Some(image.into())
            } else {
                None
            }
        };

        let audio = Audio::try_new(input, &key)?;

        Ok(Self {
            key,
            comment,
            meta,
            image,
            audio,
        })
    }

    fn read_frame(input: &mut R) -> Result<(Vec<u8>, u32)> {
        let len = Self::read_len(input)?;
        if len > 0 {
            let mut data = vec![0; len.try_into()?];
            input.read_exact(&mut data)?;
            Ok((data, len))
        } else {
            Ok((vec![], 0))
        }
    }

    fn read_len(input: &mut R) -> Result<u32> {
        let mut buffer = [0; 4];
        input.read_exact(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }

    fn skip(input: &mut R, i: usize) -> Result<()> {
        let mut buffer = vec![0; i];
        input.read_exact(&mut buffer)?;

        Ok(())
    }
}
