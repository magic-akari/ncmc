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
#[allow(dead_code)]
pub(crate) enum MusicId {
    Num(u32),
    Str(String),
}

impl de::Visitor for Place<MusicId> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        let out = match s.parse::<u32>() {
            Ok(value) => MusicId::Num(value),
            Err(..) => MusicId::Str(s.to_string()),
        };
        self.out = Some(out);

        Ok(())
    }

    fn nonnegative(&mut self, n: u64) -> miniserde::Result<()> {
        let value = n as u32;
        self.out = Some(MusicId::Num(value));

        Ok(())
    }
}

impl de::Deserialize for MusicId {
    fn begin(out: &mut Option<Self>) -> &mut dyn de::Visitor {
        Place::new(out)
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use miniserde::json;

    use super::*;

    #[testing::fixture("../ncmc/tests/input/*.json")]
    fn test_deserialize(input: PathBuf) {
        let data = fs::read(&input).unwrap();
        let meta = String::from_utf8_lossy(&data);
        match json::from_str::<MusicMeta>(&meta) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("{meta}");
                panic!("{err}");
            }
        };
    }
}
