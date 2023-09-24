mod music_meta;

use anyhow::{Ok, Result};
use id3::TagLike;
use miniserde::json;
use ncm_core::{audio::Type as AudioType, decoder::Decoder};
use std::{
    io::{Cursor, Read, Write},
    vec,
};

use crate::music_meta::MusicMeta;

const TOOL_INFO: &str = include_str!("tool_info");

pub struct Encoder {
    pub data: Vec<u8>,
    pub meta: String,
}

impl Encoder {
    pub fn encode<R>(decoder: Decoder<R>) -> Result<Self>
    where
        R: Read,
    {
        let mut buffer = vec![];
        let audio_type = decoder.audio_type();

        let Decoder { comment, meta, image, mut audio, .. } = decoder;
        let meta = String::from_utf8_lossy(&meta);

        let music_meta: MusicMeta = json::from_str(&meta)?;

        audio.read_to_end(&mut buffer)?;

        match audio_type {
            AudioType::Flac => {
                let mut tag = metaflac::Tag::read_from(&mut Cursor::new(&buffer))?;
                let data = metaflac::Tag::skip_metadata(&mut Cursor::new(&buffer));

                let vorbis_comment = tag.vorbis_comments_mut();
                vorbis_comment.set_title(vec![music_meta.music_name]);
                vorbis_comment.set_album(vec![music_meta.album]);
                vorbis_comment
                    .set_artist(music_meta.artist.into_iter().map(|ar| ar.0).collect::<Vec<_>>());
                vorbis_comment
                    .set("DESCRIPTION", vec![String::from_utf8_lossy(&comment), TOOL_INFO.into()]);
                vorbis_comment.set("TOOL", vec![TOOL_INFO]);

                if let Some(image) = image {
                    tag.add_picture(
                        image.mime_type(),
                        metaflac::block::PictureType::CoverFront,
                        image.into_data(),
                    );
                }
                buffer.clear();
                tag.remove_blocks(metaflac::BlockType::Padding);
                tag.write_to(&mut buffer)?;
                buffer.write_all(&data)?;
            }
            AudioType::Mp3 => {
                let mut tag = id3::Tag::read_from(&mut Cursor::new(&buffer))?;
                let mut data_reader = Cursor::new(&buffer);
                id3::Tag::skip(&mut data_reader)?;

                tag.set_title(music_meta.music_name);
                tag.set_album(music_meta.album);
                tag.set_artist(
                    music_meta.artist.into_iter().map(|ar| ar.0).collect::<Vec<_>>().join("/"),
                );
                tag.add_frame(id3::frame::Comment {
                    lang: "eng".into(),
                    description: "".into(),
                    text: String::from_utf8_lossy(&comment).into(),
                });
                tag.set_text("TSSE", TOOL_INFO);
                tag.set_text("TENC", TOOL_INFO);
                if let Some(image) = image {
                    tag.add_frame(id3::frame::Picture {
                        mime_type: image.mime_type().into(),
                        picture_type: id3::frame::PictureType::CoverFront,
                        description: "Cover".into(),
                        data: image.into_data(),
                    });
                }

                let mut result = vec![];
                tag.write_to(&mut result, id3::Version::Id3v24)?;
                data_reader.read_to_end(&mut result)?;
                buffer = result;
            }
            _ => {}
        }

        Ok(Self { data: buffer, meta: meta.into() })
    }
}
