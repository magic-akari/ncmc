pub mod audio;
pub mod decoder;
pub mod image;
mod key;
mod ncm_rc4;

#[cfg(test)]
mod tests {
    use crate::decoder::Decoder;
    use anyhow::Result;
    use std::{
        fs::{self, File},
        io::Read,
        path::Path,
        vec,
    };

    #[test]
    fn it_works() -> Result<()> {
        let path = "/Users/akari/Github/ncmc/samples/1.ncm";
        let input_path = Path::new(path);

        let reader = File::open(input_path)?;
        let decoder = Decoder::try_new(reader)?;

        println!("comment = {}", String::from_utf8(decoder.comment)?);
        println!("meta = {}", String::from_utf8(decoder.meta)?);

        if let Some(image) = decoder.image {
            let image_path = Path::new(path).with_extension(image.ext());

            fs::write(image_path, image.data())?;
        }

        let mut audio = decoder.audio;
        let audio_ext = audio.ext();

        println!("audio_ext: {:?}", audio_ext);

        let data_path = Path::new(path).with_extension(audio_ext);

        let mut contents = vec![];

        audio.read_to_end(&mut contents)?;

        fs::write(data_path, contents)?;

        Ok(())
    }
}
