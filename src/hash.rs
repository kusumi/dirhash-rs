use digest::Digest;
use std::io::Read;

pub const MD5: &str = "md5";
pub const SHA1: &str = "sha1";
pub const SHA224: &str = "sha224";
pub const SHA256: &str = "sha256";
pub const SHA384: &str = "sha384";
pub const SHA512: &str = "sha512";
pub const SHA512_224: &str = "sha512_224";
pub const SHA512_256: &str = "sha512_256";
pub const SHA3_224: &str = "sha3_224";
pub const SHA3_256: &str = "sha3_256";
pub const SHA3_384: &str = "sha3_384";
pub const SHA3_512: &str = "sha3_512";

const BUFFER_SIZE: usize = 65536;

pub fn get_available_hash_algo() -> [&'static str; 12] {
    [
        MD5, SHA1, SHA224, SHA256, SHA384, SHA512, SHA512_224, SHA512_256, SHA3_224, SHA3_256,
        SHA3_384, SHA3_512,
    ]
}

#[derive(Debug)]
pub enum HashObj {
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
    None,
}

pub fn new_hash(hash_algo: &str) -> HashObj {
    match hash_algo {
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
        _ => HashObj::None,
    }
}

/*
// XXX error[E0191]: the value of the associated type `OutputSize` (from trait `OutputSizeUser`) must be specified
pub fn new_hash(hash_algo: &str) -> Box<dyn digest::Digest> {
    match hash_algo {
        MD5 => Box::new(md5::Md5::new()),
        SHA1 => Box::new(sha1::Sha1::new()),
        SHA224 => Box::new(sha2::Sha224::new()),
        SHA256 => Box::new(sha2::Sha256::new()),
        SHA384 => Box::new(sha2::Sha384::new()),
        SHA512 => Box::new(sha2::Sha512::new()),
        SHA512_224 => Box::new(sha2::Sha512_224::new()),
        SHA512_256 => Box::new(sha2::Sha512_256::new()),
        SHA3_224 => Box::new(sha3::Sha3_224::new()),
        SHA3_256 => Box::new(sha3::Sha3_256::new()),
        SHA3_384 => Box::new(sha3::Sha3_384::new()),
        SHA3_512 => Box::new(sha3::Sha3_512::new()),
        _ => panic!("{}", hash_algo),
    }
}
*/

#[derive(Debug)]
pub struct HashValue {
    pub b: Vec<u8>,
    pub written: u64,
}

pub fn get_file_hash(f: &str, hash_algo: &str) -> Result<HashValue, std::io::Error> {
    let fp = std::fs::File::open(f)?;
    let mut r = std::io::BufReader::new(fp);
    get_hash(&mut r, hash_algo)
}

pub fn get_byte_hash(s: &[u8], hash_algo: &str) -> Result<HashValue, std::io::Error> {
    let mut r = std::io::BufReader::new(s);
    get_hash(&mut r, hash_algo)
}

pub fn get_string_hash(s: &str, hash_algo: &str) -> Result<HashValue, std::io::Error> {
    let mut r = std::io::BufReader::new(s.as_bytes());
    get_hash(&mut r, hash_algo)
}

pub fn get_hash<R>(
    r: &mut std::io::BufReader<R>,
    hash_algo: &str,
) -> Result<HashValue, std::io::Error>
where
    R: std::io::Read,
{
    let mut h = new_hash(hash_algo);
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

        // make this look less insane...
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
            _ => panic!("{:?}", h),
        }
    }

    // make this look less insane...
    let b = match h {
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
        _ => panic!("{:?}", h),
    };
    Ok(HashValue {
        b,
        written: written as u64,
    })
}

pub fn get_hex_sum(sum: &Vec<u8>) -> String {
    hex::encode(sum)
}
