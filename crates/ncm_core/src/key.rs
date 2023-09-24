use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyInit};
use anyhow::Result;
use ecb::Decryptor;

type Aes128EcbDec = Decryptor<aes::Aes128>;

const META_KEY: &[u8; 16] = include_bytes!("meta.key");
const CORE_KEY: &[u8; 16] = include_bytes!("core.key");

pub(crate) fn decrypt_meta(data: &mut [u8]) -> Result<&[u8]> {
    let cipher = Aes128EcbDec::new(META_KEY.into());

    cipher.decrypt_padded_mut::<Pkcs7>(data).map_err(anyhow::Error::msg)
}

pub(crate) fn decrypt_key(data: &mut [u8]) -> Result<&[u8]> {
    data.iter_mut().for_each(|byte| *byte ^= 100);

    let cipher = Aes128EcbDec::new(CORE_KEY.into());
    cipher.decrypt_padded_mut::<Pkcs7>(data).map_err(anyhow::Error::msg)
}

#[cfg(test)]
mod tests {
    use super::{decrypt_key, decrypt_meta};
    use anyhow::{Ok, Result};
    use base64::{engine::general_purpose::STANDARD as base64, Engine};

    #[test]
    fn test_decrypt_meta() -> Result<()> {
        let data = b"163 key(Don't modify):L64FU3W4YxX3ZFTmbZ+8/UR5O76NR+EAUgvWTIwJWpvcjmuEV52/6+VkX6oTlpPnD9GAL8pIt8cKuPSZpgbd6lm6aKAMrLJq4RSmmvJjBn/uU+CF2v+0QISaPrlQrQz+EDUnJkPGxwCM55NlSN5PzD+PgvUJhRQz0WY1aEvG8BRhYRnAHE+lx+EZnzD6UUTPbf/PnKHRhtwzcHkCE1cnoHSC1BkP7QQXkCMKtewr7mOlVhZnAviA0LDTXySmgokSrCj3oGaCLccWGRM3gkg6gPoBhlGkKRrUmOcglE63VjqP4lrj0b4WiAW/3HP2nuOCdAC2MHQFAhNFBs1Eku1qn8/KuBOZATD/BKsJDzADRt+GpwaW8dLD4N2Us1XNOGuMPESxAAyqWWLP19KExF4ZmdayT5ekodF+txkh0/U+CAfan3KtxAfbmGBdoE0HyKgpWlRdSoQ0YMQbKgMVyCE71vIERsKfTXE/54xUTW+U/4r+10jKhnHb5Ldam5r1seEYWiuNB1LpRmM3gPpUncYfA0NqqNyOh76YQTqurnkI3yDH9NfkcA5iR/ptOIZDvaefbZ3qaT7mOhrul58uWc61GfBZqEdiF77PsqVcyN8k2nr1VZqmLWyskTd0ahRkrJ5q";
        let mut data = base64.decode(&data[22..])?;
        let data = decrypt_meta(&mut data)?;
        let result = r#"music:{"musicId":28254848,"musicName":"「わたしは阿良々木さんと会えたんですから」","artist":[["神前暁",14629]],"albumId":2759689,"album":"「傾物語」劇伴音楽集 & あとがたり","albumPicDocId":"109951166198486505","albumPic":"https://p3.music.126.net/P3xnM1N0Cebcs3DHSUk5QA==/109951166198486505.jpg","bitrate":320000,"mp3DocId":"885f47d55947dbaea147279f20c86c9b","duration":90331,"mvId":0,"alias":[],"transNames":[],"format":"mp3"}"# .as_bytes();
        assert_eq!(data, result);

        let data = b"163 key(Don't modify):L64FU3W4YxX3ZFTmbZ+8/XG3Yan7ukb4b51wQRVNwugbIH6ezOeqj930fJXsqInFzqcsHXpQ7+sTfxYhUpuytWYiHNRfULDK/Fa4CYDdIS++hoiz+fzS/vigPeEz9DVzFVCigQVQ7bU6aoNpxnTfoGH0dc14woq4w3zNbOXSV1+dPAfltLghy3ZNy+z7gOXyriEAPTrovCwuaLLKNKJ1MCmiVpyAaTFDCOIAfWHEadhZf/FGXnMM1WWDbFyAG2nezOgS4F5ods4RZ+S5PUPcI86nLB16UO/rE38WIVKbsrVmIhzUX1CwyvxWuAmA3SEvzex/uem1sKTGrsv3PG0Z2uMqMZ6LgcWu66WDTaZfmK+c+LsdmrfDV3EXBEZ4IXuLv1Vyr0+l5FBmXF/7oEfXwnOvGdUqoUItNzQrjMRnkuABE84Dhm410WVAV6KGWoGqwlbW34Qx/0CKoSx4NTiGqywQ/2I1WN4cR3HBJ5wZJTlJEYxDaYXgYJBWlJk5RaIFhXg31XVg25hKuFlVEmMSpS3PLgtEX/kJhRBwIRX/fASSe3qfVjVRRo6IHWVQEPplmJSi0Lr+CfVZymJIF/TqidD4cb4+7rDjZd8DrfWHPgG92M/2Q3llscQmdLqe7HbiM0EnwIe7E9w9H9lwBk0InO6Q6vkN9QPo64i9YbPKQdNzguu45pDh4jhr1ofTNKiGluqqS+MdX+Kv1+rMlkv8PosDD9N4T9F8F8v8OQvxQDJp4SQAVTdL6+EDDxj7nIMZ";
        let mut data = base64.decode(&data[22..])?;
        let data = decrypt_meta(&mut data)?;
        let result = r#"music:{"musicId":1483150397,"musicName":"なかよし!〇!なかよし!<TVサイズ>","artist":[["水瀬いのり",1004106],["徳井青空",740113],["村川梨衣",862037]],"albumId":96108680,"album":"なかよし!〇!なかよし!<TVサイズ>","albumPicDocId":"109951165351519138","albumPic":"https://p3.music.126.net/Pu2wRB2EsQBj9LiFCUpaaQ==/109951165351519138.jpg","bitrate":128004,"mp3DocId":"82fb055351dd95380841bf290527b6e1","duration":89000,"mvId":0,"alias":["TV动画《请问您今天要来点兔子吗？ BLOOM》片尾曲"],"transNames":[],"format":"mp3"}"# .as_bytes();
        assert_eq!(data, result);

        let data = b"163 key(Don't modify):L64FU3W4YxX3ZFTmbZ+8/cH1s2iVmOtq9z5+98DLWLxyhvbTNVJ4gPtpzA1DOnu0LTwYkoIZ0ZOU0hcDEuMA4WEmQraNcvQyi6y71+LqG7prWWgg17gQPc/be6XbV+dDYrXq0Pmn3h62bKp6dgHv/xOfcAXP0Tje6KSXq8bXMTSBlsbr2UINd7DcJsttJX4oWN0GYzmkGKCaZJ3crOBvN15XmMHkHA6lnvz2FeoZ3Bj1tu1be7JesfCO7iI4EDph5q6BlPJRf+bTmH0STg/SHiroLNkQTPTv+0C1Dc8vhuHkUjXxsj3SG+SIAFYhKT7eOKAp1+grFGiQ4WKVIOkHh7VZ2VrXzWmxqA15W9hqo+Yfq/Uv8gj1R7n0y6jCFnbKdwH/gFsQa6bNaf98s24dPPXVgIOKjCh7k2ZFwxSg5M8LpCGNAA4x3Pymv1TjrmzjAFLEcLRIrTqBqlT2LELPQhmXm5+kTV6SbIAv88u+opztVl9Iz7CPgifKJPcLZ/yb4pt6BJBKBhCi6gzig7CllsA99W3S3YteoCWiaKh1dtSz3OURqk22sObLKAD8X9EDul4hfw4QXWNcGe059utso0vDkdkNYhz4uTW38PfuFBv9TLwINTiJpfI2i9+35Yh8ab78Ua7L/H6xjA2/x3fxHaywi4SypxwQOc9S/+QUi9s=";
        let mut data = base64.decode(&data[22..])?;
        let data = decrypt_meta(&mut data)?;
        let result = r#"music:{"musicId":1483146611,"musicName":"天空カフェテリア<TVサイズ>","artist":[["Petit Rabbit's",939023]],"albumId":96107824,"album":"天空カフェテリア<TVサイズ>","albumPicDocId":"109951165351498889","albumPic":"https://p4.music.126.net/eWfBZq8nguEwX_mBAzdh6Q==/109951165351498889.jpg","bitrate":128004,"mp3DocId":"26d870a21a2e995c657c9882df670789","duration":89000,"mvId":0,"alias":["TV动画《请问您今天要来点兔子吗？ BLOOM》片头曲"],"transNames":[],"format":"mp3"}"# .as_bytes();
        assert_eq!(data, result);

        Ok(())
    }

    #[test]
    fn test_decrypt_key() -> Result<()> {
        let mut data = [
            44, 206, 213, 235, 105, 234, 251, 20, 85, 13, 69, 191, 97, 221, 23, 29, 18, 21, 246,
            51, 21, 69, 239, 181, 244, 6, 32, 104, 211, 135, 179, 202, 61, 32, 50, 218, 231, 3, 85,
            33, 145, 113, 162, 72, 254, 143, 136, 190, 39, 101, 206, 78, 98, 101, 116, 215, 145,
            212, 45, 190, 39, 210, 209, 192, 188, 100, 141, 243, 48, 248, 186, 138, 64, 32, 93, 82,
            40, 16, 113, 0, 51, 165, 195, 243, 34, 188, 96, 243, 168, 178, 144, 252, 165, 149, 122,
            109, 247, 164, 233, 37, 82, 40, 214, 0, 159, 5, 178, 114, 243, 218, 126, 60, 20, 5,
            164, 198, 166, 244, 88, 15, 95, 132, 197, 175, 252, 215, 77, 30,
        ];
        let data = decrypt_key(&mut data)?;
        let result = b"neteasecloudmusic143621215014397E7fT49x7dof9OKCgg9cdvhEuezy3iZCL1nFvBFd1T4uSktAJKmwZXsijPbijliionVUXXg9plTbXEclAE9Lb";
        assert_eq!(data, result);

        let mut data = [
            44, 206, 213, 235, 105, 234, 251, 20, 85, 13, 69, 191, 97, 221, 23, 29, 24, 14, 183,
            247, 76, 59, 185, 79, 73, 164, 185, 196, 134, 23, 6, 186, 205, 190, 47, 224, 57, 245,
            187, 112, 244, 6, 211, 107, 115, 146, 17, 143, 13, 144, 220, 241, 17, 53, 159, 76, 0,
            187, 138, 5, 160, 191, 178, 239, 153, 3, 26, 54, 245, 223, 12, 151, 218, 132, 221, 37,
            73, 215, 3, 216, 37, 245, 224, 82, 199, 78, 169, 157, 3, 0, 232, 180, 51, 200, 22, 175,
            103, 193, 201, 182, 68, 10, 133, 216, 144, 86, 0, 36, 43, 84, 172, 211, 192, 157, 186,
            29, 231, 137, 92, 68, 121, 212, 12, 108, 112, 23, 163, 14, 252, 18, 154, 195, 161, 219,
            220, 182, 182, 222, 155, 40, 178, 65, 51, 212,
        ];
        let data = decrypt_key(&mut data)?;
        let result = b"neteasecloudmusic20760577722136161195829593819E7fT49x7dof9OKCgg9cdvhEuezy3iZCL1nFvBFd1T4uSktAJKmwZXsijPbijliionVUXXg9plTbXEclAE9Lb";
        assert_eq!(data, result);

        let mut data = [
            44, 206, 213, 235, 105, 234, 251, 20, 85, 13, 69, 191, 97, 221, 23, 29, 50, 98, 84,
            233, 179, 77, 58, 247, 251, 194, 99, 142, 70, 30, 119, 85, 132, 69, 234, 136, 179, 165,
            196, 182, 197, 116, 69, 53, 226, 197, 55, 119, 172, 10, 194, 224, 47, 182, 226, 143,
            136, 194, 120, 104, 104, 71, 126, 237, 102, 235, 23, 97, 245, 89, 170, 79, 209, 120, 7,
            232, 169, 106, 14, 199, 248, 125, 250, 37, 240, 177, 39, 46, 62, 13, 16, 47, 105, 32,
            156, 243, 120, 197, 44, 177, 210, 188, 96, 236, 187, 37, 132, 179, 184, 203, 205, 93,
            30, 57, 75, 250, 29, 176, 0, 59, 216, 67, 169, 114, 110, 128, 82, 47, 10, 156, 9, 66,
            164, 56, 90, 51, 125, 4, 184, 133, 86, 188, 99, 103,
        ];
        let data = decrypt_key(&mut data)?;
        let result = b"neteasecloudmusic7759998725616604731636941146E7fT49x7dof9OKCgg9cdvhEuezy3iZCL1nFvBFd1T4uSktAJKmwZXsijPbijliionVUXXg9plTbXEclAE9Lb";
        assert_eq!(data, result);

        Ok(())
    }
}
