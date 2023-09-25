use anyhow::{Context, Result};
use bpaf::Bpaf;
use ncm_core::decoder::Decoder;
use ncm_meta::Encoder;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
struct Opts {
    #[bpaf(external, fallback(Mode::Auto))]
    mode: Mode,

    #[bpaf(positional("INPUT"))]
    input: Vec<PathBuf>,
}

#[derive(Debug, Clone, Bpaf, Default)]
enum Mode {
    /// convert files with metadata, this is the default mode
    #[default]
    Auto,
    /// dump all data
    Dump,
}

fn main() -> Result<()> {
    let opts = opts().run();

    match opts.mode {
        Mode::Auto => auto(&opts.input),
        Mode::Dump => dump(&opts.input),
    }
}

fn auto(input_list: &[PathBuf]) -> Result<()> {
    for path in input_list {
        let reader = fs::File::open(path).with_context(|| format!("input {}", path.display()))?;
        let decoder = Decoder::decode(reader)?;
        let ext = decoder.ext();
        let output = Path::new(&path).with_extension(ext);

        println!("{}", output.display());

        let Encoder { data, meta } = Encoder::encode(decoder)?;

        eprintln!("{meta}");

        fs::write(output, data)?;
    }

    anyhow::Ok(())
}

fn dump(input_list: &[PathBuf]) -> Result<()> {
    for path in input_list {
        let reader = fs::File::open(path).with_context(|| format!("input {}", path.display()))?;
        println!("{}", path.display());

        let Decoder { key, comment, meta, image, mut audio } = Decoder::decode(reader)?;

        {
            let meta = if !meta.is_empty() {
                String::from_utf8_lossy(&meta)
            } else {
                "meta not found".into()
            };
            eprintln!("{meta}");
        }

        let key_path = path.with_extension("key");
        fs::write(key_path, key)?;

        if !comment.is_empty() {
            let comment_path = path.with_extension("comment");
            fs::write(comment_path, comment)?;
        }

        if !meta.is_empty() {
            let meta_path = path.with_extension("json");
            fs::write(meta_path, meta)?;
        }

        if let Some(image) = image {
            let image_path = path.with_extension(image.ext());
            fs::write(image_path, image.data())?;
        }

        let audio_path = path.with_extension(audio.ext());

        let mut file = fs::File::options().write(true).open(audio_path)?;

        io::copy(&mut audio, &mut file)?;
    }

    anyhow::Ok(())
}
