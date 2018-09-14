extern crate aes;
extern crate base64;
extern crate block_modes;
extern crate glob;
extern crate id3;
extern crate metaflac;

#[macro_use]
extern crate miniserde;

use miniserde::json;

use std::error;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::str;
use std::{io, io::prelude::*};

use aes::Aes128;
use base64::decode;
use block_modes::{block_padding::Pkcs7, BlockMode, Ecb};

// CTENFDAM
const MAGIC_HEADER: [u8; 8] = [0x43, 0x54, 0x45, 0x4e, 0x46, 0x44, 0x41, 0x4d];

const CORE_KEY: [u8; 16] = [
    0x68, 0x7A, 0x48, 0x52, 0x41, 0x6D, 0x73, 0x6F, //
    0x35, 0x6B, 0x49, 0x6E, 0x62, 0x61, 0x78, 0x57, //
];

const META_KEY: [u8; 16] = [
    0x23, 0x31, 0x34, 0x6C, 0x6A, 0x6B, 0x5F, 0x21, //
    0x5C, 0x5D, 0x26, 0x30, 0x55, 0x3C, 0x27, 0x28, //
];

const BUFFER_SIZE: usize = 0x8000;

#[derive(Debug)]
struct SimpleError<'a>(&'a str);

impl<'a> fmt::Display for SimpleError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> error::Error for SimpleError<'a> {
    fn description(&self) -> &str {
        self.0
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[derive(Debug, MiniDeserialize)]
struct MusicMeta {
    #[serde(rename = "musicId")]
    music_id: u32,
    #[serde(rename = "musicName")]
    music_name: String,
    artist: Vec<(String, u32)>,
    album: String,
    format: String,
}

#[inline]
fn get_u32(buffer: &[u8]) -> u32 {
    assert!(buffer.len() >= 4);
    u32::from(buffer[0])
        | u32::from(buffer[1]) << 8
        | u32::from(buffer[2]) << 16
        | u32::from(buffer[3]) << 24
}

pub fn convert(file_path: PathBuf) -> Result<PathBuf, Box<error::Error>> {
    let mut input = io::BufReader::new(File::open(&file_path)?);
    let mut buffer = [0; BUFFER_SIZE];
    input.read_exact(&mut buffer[..8])?;

    if buffer[..8] != MAGIC_HEADER {
        return Err(Box::new(SimpleError("invalid file header")));
    }

    // input.seek_relative(2);
    input.seek(io::SeekFrom::Current(2))?;

    input.read_exact(&mut buffer[0..4])?;

    let key_len = get_u32(&buffer[..4]);

    let mut key_data = vec![0; key_len as usize];

    input.read_exact(&mut key_data)?;

    for data in &mut key_data {
        *data ^= 0x64;
    }

    type Aes128Ecb = Ecb<Aes128, Pkcs7>;

    let cipher = Aes128Ecb::new_varkey(&CORE_KEY).unwrap();

    let de_key_data = &cipher.decrypt_pad(&mut key_data).unwrap()[17..];

    let key_len = de_key_data.len();

    let mut key_box = (0u8..=255).collect::<Vec<_>>();

    let mut j: usize = 0;

    for i in 0..256 {
        j = (key_box[i] as usize + j + de_key_data[i % key_len] as usize) & 0xff;
        key_box.swap(i, j);
    }

    let mut new_key_box = [0u8; 256];

    for (i, item) in new_key_box.iter_mut().enumerate() {
        *item = key_box
            [(key_box[i] as usize + key_box[(((key_box[i]) as usize) + i) & 0xff] as usize) & 0xff];
    }

    input.read_exact(&mut buffer[0..4])?;

    let meta_data_len = get_u32(&buffer[..4]);

    let mut meta_data = vec![0; meta_data_len as usize];

    input.read_exact(&mut meta_data)?;

    for data in &mut meta_data {
        *data ^= 0x63;
    }

    // meta_data == "163 key(Don't modify):" + base64 string

    let cipher = Aes128Ecb::new_varkey(&META_KEY).unwrap();

    let mut bytes = decode(&meta_data[22..]).unwrap();

    let meta_data_decoded = cipher.decrypt_pad(&mut bytes).unwrap();

    // meta_data_decoded == "music:" + json string

    let music_meta: MusicMeta = json::from_str(str::from_utf8(&meta_data_decoded[6..])?)?;

    println!("{:>10}\t{}", "musicId", music_meta.music_id);
    println!("{:>10}\t{}", "musicName", music_meta.music_name);
    println!("{:>10}\t{}", "album", music_meta.album);
    println!("{:>10}\t{}", "format", music_meta.format);

    // input.read_exact(&mut buffer[0..4])?;

    // let crc32 = buffer[0] as u32
    //     | (buffer[1] as u32) << 8
    //     | (buffer[2] as u32) << 16
    //     | (buffer[3] as u32) << 24;

    // println!("crc32 = {:?}", crc32);

    // input.seek_relative(5);
    input.seek(io::SeekFrom::Current(9))?;

    input.read_exact(&mut buffer[0..4])?;

    let image_size = get_u32(&buffer[..4]);

    let mut image = vec![0; image_size as usize];
    input.read_exact(&mut image)?;

    let image_mime_type = match &image[..8] {
        [137, 80, 78, 71, 13, 10, 26, 10] => "image/png",
        [0xFF, 0xD8, 0xFF, 0xE0, _, _, _, _] => "image/jpeg",
        [71, 73, 70, _, _, _, _, _] => "image/gif",
        _ => "image/*",
    };

    let mut target_path = file_path;
    target_path.set_extension(&music_meta.format);

    {
        let mut output = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&target_path)?;

        loop {
            let read_size = input.read(&mut buffer)?;
            if read_size == 0 {
                break;
            }
            for (i, item) in buffer.iter_mut().enumerate().take(read_size) {
                let j = (i + 1) & 0xff;
                *item ^= new_key_box[j];
            }

            output.write_all(&buffer)?;
        }
    }

    match &music_meta.format[..] {
        "flac" => {
            let mut tag = metaflac::Tag::read_from_path(&target_path)?;
            {
                let mut vorbis_comment = tag.vorbis_comments_mut();
                vorbis_comment.set_title(vec![music_meta.music_name]);
                vorbis_comment.set_album(vec![music_meta.album]);
                vorbis_comment.set_artist(
                    music_meta
                        .artist
                        .into_iter()
                        .map(|ar| ar.0)
                        .collect::<Vec<_>>(),
                );
            }

            tag.add_picture(
                image_mime_type,
                metaflac::block::PictureType::CoverFront,
                image,
            );
            tag.save()?;
        }
        "mp3" => {
            let mut tag = id3::Tag::read_from_path(&target_path)?;
            tag.set_title(music_meta.music_name);
            tag.set_album(music_meta.album);
            tag.set_artist(
                music_meta
                    .artist
                    .into_iter()
                    .map(|ar| ar.0)
                    .collect::<Vec<_>>()
                    .join("/"),
            );

            // tag.add_comment(id3::frame::Comment {
            //     lang: "XXX".to_string(),
            //     description: "".to_string(),
            //     text: String::from_utf8(meta_data)?,
            // });

            tag.add_picture(id3::frame::Picture {
                mime_type: image_mime_type.to_string(),
                picture_type: id3::frame::PictureType::CoverFront,
                description: "Cover".to_string(),
                data: image,
            });
            tag.write_to_path(&target_path, id3::Version::Id3v24)?;
        }
        _ => unimplemented!(),
    }

    Ok(target_path)
}
