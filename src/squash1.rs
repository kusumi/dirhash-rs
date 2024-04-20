#[cfg(feature = "squash1")]
use crate::hash;

pub(crate) const SQUASH_LABEL: &str = "squash";
pub(crate) const SQUASH_VERSION: i32 = 1;

#[derive(Debug, Default)]
pub(crate) struct Squash {
    buffer: Vec<Vec<u8>>,
}

impl Squash {
    pub(crate) fn new() -> Self {
        let mut squash = Self {
            ..Default::default()
        };
        squash.init_buffer();
        squash
    }

    pub(crate) fn init_buffer(&mut self) {
        self.buffer.clear();
    }

    pub(crate) fn update_buffer(&mut self, b: &[u8]) -> std::io::Result<()> {
        let (b, ..) = hash::get_byte_hash(b, hash::MD5)?;
        self.buffer.push(b);
        Ok(())
    }

    pub(crate) fn get_buffer(&self) -> Vec<u8> {
        // XXX directly sort Vec<Vec<u8>>
        let mut s = Vec::new();
        for v in &self.buffer {
            s.push(hash::get_hex_sum(v));
        }
        s.sort();
        s.join("").as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_init_buffer() {
        let squash = super::Squash::new();
        assert!(squash.get_buffer().is_empty());
    }

    #[test]
    fn test_update_buffer() {
        let mut squash = super::Squash::new();

        if let Err(e) = squash.update_buffer(&[]) {
            panic!("{e}");
        }
        assert!(!squash.get_buffer().is_empty());

        if let Err(e) = squash.update_buffer(&[]) {
            panic!("{e}");
        }
        assert!(!squash.get_buffer().is_empty());

        if let Err(e) = squash.update_buffer("xxx".as_bytes()) {
            panic!("{e}");
        }
        assert!(!squash.get_buffer().is_empty());

        if let Err(e) = squash.update_buffer("x".repeat(123_456).as_bytes()) {
            panic!("{e}");
        }
        assert!(!squash.get_buffer().is_empty());
    }
}
