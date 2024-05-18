use ncm_core::decoder::Decoder;
use std::{fs, path::PathBuf};

#[testing::fixture("tests/input/*.ncm")]
fn test_dump(input: PathBuf) {
    let reader = fs::File::open(&input).unwrap();

    let Decoder { key, comment, meta, image, .. } = Decoder::decode(reader).unwrap();

    if !meta.is_empty() {
        let meta = String::from_utf8_lossy(&meta);
        eprintln!("{meta}");
    };

    let key_path = input.with_extension("key");
    let expected_key = fs::read(key_path).unwrap();
    assert_eq!(key, expected_key);

    if !comment.is_empty() {
        let comment_path = input.with_extension("comment");
        let expected_comment = fs::read(comment_path).unwrap();
        assert_eq!(comment, expected_comment);
    }

    if !meta.is_empty() {
        let meta_path = input.with_extension("json");
        let expected_meta = fs::read(meta_path).unwrap();
        assert_eq!(meta, expected_meta);
    }

    if let Some(image) = image {
        let image_path = input.with_extension(image.ext());
        let expected_image = fs::read(image_path).unwrap();
        assert_eq!(image.data(), &expected_image);
    }
}
