use digest::Digest;

pub(crate) const MD5: &str = "md5";
pub(crate) const SHA1: &str = "sha1";
pub(crate) const SHA224: &str = "sha224";
pub(crate) const SHA256: &str = "sha256";
pub(crate) const SHA384: &str = "sha384";
pub(crate) const SHA512: &str = "sha512";
pub(crate) const SHA512_224: &str = "sha512_224";
pub(crate) const SHA512_256: &str = "sha512_256";
pub(crate) const SHA3_224: &str = "sha3_224";
pub(crate) const SHA3_256: &str = "sha3_256";
pub(crate) const SHA3_384: &str = "sha3_384";
pub(crate) const SHA3_512: &str = "sha3_512";

const BUFFER_SIZE: usize = 65536;

pub(crate) fn get_available_hash_algo() -> [&'static str; 12] {
    [
        MD5, SHA1, SHA224, SHA256, SHA384, SHA512, SHA512_224, SHA512_256, SHA3_224, SHA3_256,
        SHA3_384, SHA3_512,
    ]
}

#[derive(Debug)]
pub(crate) enum HashObj {
    MD5(md5::Md5),
    SHA1(sha1::Sha1),
    SHA224(sha2::Sha224),
    SHA256(sha2::Sha256),
    SHA384(sha2::Sha384),
    SHA512(sha2::Sha512),
    SHA512_224(sha2::Sha512_224),
    SHA512_256(sha2::Sha512_256),
    SHA3_224(sha3::Sha3_224),
    SHA3_256(sha3::Sha3_256),
    SHA3_384(sha3::Sha3_384),
    SHA3_512(sha3::Sha3_512),
}

pub(crate) fn new_hash(hash_algo: &str) -> std::io::Result<HashObj> {
    Ok(match hash_algo {
        MD5 => HashObj::MD5(md5::Md5::new()),
        SHA1 => HashObj::SHA1(sha1::Sha1::new()),
        SHA224 => HashObj::SHA224(sha2::Sha224::new()),
        SHA256 => HashObj::SHA256(sha2::Sha256::new()),
        SHA384 => HashObj::SHA384(sha2::Sha384::new()),
        SHA512 => HashObj::SHA512(sha2::Sha512::new()),
        SHA512_224 => HashObj::SHA512_224(sha2::Sha512_224::new()),
        SHA512_256 => HashObj::SHA512_256(sha2::Sha512_256::new()),
        SHA3_224 => HashObj::SHA3_224(sha3::Sha3_224::new()),
        SHA3_256 => HashObj::SHA3_256(sha3::Sha3_256::new()),
        SHA3_384 => HashObj::SHA3_384(sha3::Sha3_384::new()),
        SHA3_512 => HashObj::SHA3_512(sha3::Sha3_512::new()),
        _ => return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput)),
    })
}

pub(crate) fn get_file_hash(f: &str, hash_algo: &str) -> std::io::Result<(Vec<u8>, u64)> {
    let mut r = std::io::BufReader::new(std::fs::File::open(f)?);
    get_hash(&mut r, hash_algo)
}

pub(crate) fn get_byte_hash(s: &[u8], hash_algo: &str) -> std::io::Result<(Vec<u8>, u64)> {
    let mut r = std::io::BufReader::new(s);
    get_hash(&mut r, hash_algo)
}

pub(crate) fn get_string_hash(s: &str, hash_algo: &str) -> std::io::Result<(Vec<u8>, u64)> {
    let mut r = std::io::BufReader::new(s.as_bytes());
    get_hash(&mut r, hash_algo)
}

/* XXX which style to use ?
pub(crate) fn get_hash<R>(
    r: &mut std::io::BufReader<R>,
    hash_algo: &str,
) -> std::io::Result<(Vec<u8>, u64)>
where
    R: std::io::Read,
*/
pub(crate) fn get_hash(
    r: &mut impl std::io::BufRead,
    hash_algo: &str,
) -> std::io::Result<(Vec<u8>, u64)> {
    let mut h = new_hash(hash_algo)?;
    let mut written = 0;

    loop {
        let mut buf = [0; BUFFER_SIZE];
        let ret = match r.read(&mut buf) {
            Ok(v) => match v {
                0 => break,
                _ => v,
            },
            Err(e) => return Err(e),
        };
        let b = &buf[..ret];
        written += b.len();
        match h {
            HashObj::MD5(ref mut v) => v.update(b),
            HashObj::SHA1(ref mut v) => v.update(b),
            HashObj::SHA224(ref mut v) => v.update(b),
            HashObj::SHA256(ref mut v) => v.update(b),
            HashObj::SHA384(ref mut v) => v.update(b),
            HashObj::SHA512(ref mut v) => v.update(b),
            HashObj::SHA512_224(ref mut v) => v.update(b),
            HashObj::SHA512_256(ref mut v) => v.update(b),
            HashObj::SHA3_224(ref mut v) => v.update(b),
            HashObj::SHA3_256(ref mut v) => v.update(b),
            HashObj::SHA3_384(ref mut v) => v.update(b),
            HashObj::SHA3_512(ref mut v) => v.update(b),
        }
    }

    Ok((
        match h {
            HashObj::MD5(v) => v.finalize()[..].to_vec(),
            HashObj::SHA1(v) => v.finalize()[..].to_vec(),
            HashObj::SHA224(v) => v.finalize()[..].to_vec(),
            HashObj::SHA256(v) => v.finalize()[..].to_vec(),
            HashObj::SHA384(v) => v.finalize()[..].to_vec(),
            HashObj::SHA512(v) => v.finalize()[..].to_vec(),
            HashObj::SHA512_224(v) => v.finalize()[..].to_vec(),
            HashObj::SHA512_256(v) => v.finalize()[..].to_vec(),
            HashObj::SHA3_224(v) => v.finalize()[..].to_vec(),
            HashObj::SHA3_256(v) => v.finalize()[..].to_vec(),
            HashObj::SHA3_384(v) => v.finalize()[..].to_vec(),
            HashObj::SHA3_512(v) => v.finalize()[..].to_vec(),
        },
        written.try_into().unwrap(),
    ))
}

pub(crate) fn get_hex_sum(sum: &[u8]) -> String {
    hex::encode(sum)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new_hash() {
        for s in &super::get_available_hash_algo() {
            if let Err(e) = super::new_hash(s) {
                panic!("{e:?}");
            }
        }

        let invalid_list = ["", "xxx", "SHA256", "516e7cb4-6ecf-11d6-8ff8-00022d09712b"];
        for s in &invalid_list {
            if let Ok(v) = super::new_hash(s) {
                panic!("{v:?}");
            }
        }
    }

    #[test]
    fn test_get_byte_hash() {
        let alg_sum_list_1 = [
            (super::MD5, "d41d8cd98f00b204e9800998ecf8427e"),
            (super::SHA1, "da39a3ee5e6b4b0d3255bfef95601890afd80709"),
            (super::SHA224, "d14a028c2a3a2bc9476102bb288234c415a2b01f828ea62ac5b3e42f"),
            (super::SHA256, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
            (super::SHA384, "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b"),
            (super::SHA512, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e"),
        ];
        for x in &alg_sum_list_1 {
            match super::get_byte_hash(&[], x.0) {
                Ok(v) => {
                    assert_eq!(super::get_hex_sum(&v.0), x.1);
                    assert_eq!(v.1, 0);
                }
                Err(e) => panic!("{e}"),
            }
        }

        let alg_sum_list_2 = [
            (super::MD5, "48fcdb8b87ce8ef779774199a856091d"),
            (super::SHA1, "065e431442d313aa4c4345f1c7f3d3a84a9b201f"),
            (super::SHA224, "62f2929306a761f06a3b055aac36ec38df8e275a8b66e68c52f030d3"),
            (super::SHA256, "e23c0cda5bcdecddec446b54439995c7260c8cdcf2953eec9f5cdb6948e5898d"),
            (super::SHA384, "3a52aaed14b5b6f9f7208914e5c34f0e16e70a285c37fd964ab918980a40acb52be0a71d43cdabb702aa2d025ce9ab7b"),
            (super::SHA512, "990fed5cd10a549977ef6c9e58019a467f6c7aadffb9a6d22b2d060e6989a06d5beb473ebc217f3d553e16bf482efdc4dd91870e7943723fdc387c2e9fa3a4b8"),
        ];
        let s = "A".repeat(1_000_000);
        for x in &alg_sum_list_2 {
            match super::get_byte_hash(s.as_bytes(), x.0) {
                Ok(v) => {
                    assert_eq!(super::get_hex_sum(&v.0), x.1);
                    assert_eq!(v.1, 1_000_000);
                }
                Err(e) => panic!("{e}"),
            }
        }
    }

    #[test]
    fn test_get_string_hash() {
        let alg_sum_list_1 = [
            (super::MD5, "d41d8cd98f00b204e9800998ecf8427e"),
            (super::SHA1, "da39a3ee5e6b4b0d3255bfef95601890afd80709"),
            (super::SHA224, "d14a028c2a3a2bc9476102bb288234c415a2b01f828ea62ac5b3e42f"),
            (super::SHA256, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
            (super::SHA384, "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b"),
            (super::SHA512, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e"),
        ];
        for x in &alg_sum_list_1 {
            match super::get_string_hash("", x.0) {
                Ok(v) => {
                    assert_eq!(super::get_hex_sum(&v.0), x.1);
                    assert_eq!(v.1, 0);
                }
                Err(e) => panic!("{e}"),
            }
        }

        let alg_sum_list_2 = [
            (super::MD5, "48fcdb8b87ce8ef779774199a856091d"),
            (super::SHA1, "065e431442d313aa4c4345f1c7f3d3a84a9b201f"),
            (super::SHA224, "62f2929306a761f06a3b055aac36ec38df8e275a8b66e68c52f030d3"),
            (super::SHA256, "e23c0cda5bcdecddec446b54439995c7260c8cdcf2953eec9f5cdb6948e5898d"),
            (super::SHA384, "3a52aaed14b5b6f9f7208914e5c34f0e16e70a285c37fd964ab918980a40acb52be0a71d43cdabb702aa2d025ce9ab7b"),
            (super::SHA512, "990fed5cd10a549977ef6c9e58019a467f6c7aadffb9a6d22b2d060e6989a06d5beb473ebc217f3d553e16bf482efdc4dd91870e7943723fdc387c2e9fa3a4b8"),
        ];
        let s = "A".repeat(1_000_000);
        for x in &alg_sum_list_2 {
            match super::get_string_hash(&s, x.0) {
                Ok(v) => {
                    assert_eq!(super::get_hex_sum(&v.0), x.1);
                    assert_eq!(v.1, 1_000_000);
                }
                Err(e) => panic!("{e}"),
            }
        }
    }
}
