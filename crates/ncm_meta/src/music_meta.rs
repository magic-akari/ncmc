use miniserde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
#[serde(default)]
pub(crate) struct MusicMeta {
    #[serde(rename = "musicId")]
    pub music_id: String, 
    #[serde(rename = "musicName")]
    pub music_name: String,
    pub artist: Vec<(String, String)>,
    pub album: String,
    #[serde(rename = "albumPic")]
    pub album_pic: String,
    pub format: String,
}
