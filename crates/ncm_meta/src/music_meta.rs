use miniserde::{de, make_place, Deserialize};

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub(crate) struct MusicMeta {
    #[serde(rename = "musicId")]
    pub music_id: MusicId,
    #[serde(rename = "musicName")]
    pub music_name: String,
    pub artist: Vec<(String, MusicId)>,
    pub album: String,
    #[serde(rename = "albumPic")]
    pub album_pic: String,
    pub format: String,
}

make_place!(Place);
#[derive(Debug)]
pub(crate) struct MusicId(u32);

impl de::Visitor for Place<MusicId> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        let value: u32 = s.parse().map_err(|_| miniserde::Error)?;
        self.out = Some(MusicId(value));

        Ok(())
    }

    fn nonnegative(&mut self, n: u64) -> miniserde::Result<()> {
        let value = n as u32;
        self.out = Some(MusicId(value));

        Ok(())
    }
}

impl de::Deserialize for MusicId {
    fn begin(out: &mut Option<Self>) -> &mut dyn de::Visitor {
        Place::new(out)
    }
}
