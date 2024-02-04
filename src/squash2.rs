#[cfg(feature = "squash2")]
use crate::hash;

pub(crate) const SQUASH_LABEL: &str = "squash";
pub(crate) const SQUASH_VERSION: i32 = 2;

#[derive(Debug, Default)]
pub(crate) struct Squash {
    squash_buffer: Vec<u8>,
}

impl Squash {
    pub(crate) fn new() -> Squash {
        let mut squash = Squash {
            ..Squash::default()
        };
        squash.init_squash_buffer();
        squash
    }

    pub(crate) fn init_squash_buffer(&mut self) {
        self.squash_buffer.clear();
    }

    pub(crate) fn update_squash_buffer(&mut self, b: &[u8]) -> std::io::Result<()> {
        // result depends on append order
        self.squash_buffer.extend(b);
        let hash::HashValue { b, .. } = hash::get_byte_hash(&self.squash_buffer, hash::SHA1)?;
        self.squash_buffer = b;
        Ok(())
    }

    pub(crate) fn get_squash_buffer(&self) -> Vec<u8> {
        self.squash_buffer.clone()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_init_squash_buffer() {
        let squash = super::Squash::new();
        assert!(squash.get_squash_buffer().is_empty());
    }

    #[test]
    fn test_update_squash_buffer() {
        let mut squash = super::Squash::new();

        if let Err(e) = squash.update_squash_buffer(&[]) {
            panic!("{}", e);
        }
        assert!(!squash.get_squash_buffer().is_empty());

        if let Err(e) = squash.update_squash_buffer(&[]) {
            panic!("{}", e);
        }
        assert!(!squash.get_squash_buffer().is_empty());

        if let Err(e) = squash.update_squash_buffer("xxx".as_bytes()) {
            panic!("{}", e);
        }
        assert!(!squash.get_squash_buffer().is_empty());

        if let Err(e) = squash.update_squash_buffer("x".repeat(123456).as_bytes()) {
            panic!("{}", e);
        }
        assert!(!squash.get_squash_buffer().is_empty());
    }
}
