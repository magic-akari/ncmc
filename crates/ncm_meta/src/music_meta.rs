use miniserde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub(crate) struct MusicMeta {
    #[serde(rename = "musicId")]
    pub music_id: u32,
    #[serde(rename = "musicName")]
    pub music_name: String,
    pub artist: Vec<(String, u32)>,
    pub album: String,
    #[serde(rename = "albumPic")]
    pub album_pic: String,
    pub format: String,
}
