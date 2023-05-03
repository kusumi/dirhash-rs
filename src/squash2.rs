#[cfg(feature = "squash2")]
use crate::hash;

pub const SQUASH_LABEL: &str = "squash";
pub const SQUASH_VERSION: i32 = 2;

#[derive(Debug, Default)]
pub struct Squash {
    squash_buffer: Vec<u8>,
}

impl Squash {
    #[allow(dead_code)]
    fn new() -> Squash {
        let mut squash = Squash {
            ..Squash::default()
        };
        squash.init_squash_buffer();
        squash
    }

    pub fn init_squash_buffer(&mut self) {
        self.squash_buffer.clear();
    }

    pub fn update_squash_buffer(&mut self, b: &[u8]) {
        // result depends on append order
        self.squash_buffer.extend(b);
        match hash::get_byte_hash(&self.squash_buffer, hash::SHA1) {
            Ok(v) => {
                let hash::HashValue { b, .. } = v;
                self.squash_buffer = b;
            }
            Err(e) => panic!("{}", e),
        }
    }

    pub fn get_squash_buffer(&self) -> Vec<u8> {
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

        squash.update_squash_buffer(&[]);
        assert!(!squash.get_squash_buffer().is_empty());

        squash.update_squash_buffer(&[]);
        assert!(!squash.get_squash_buffer().is_empty());

        squash.update_squash_buffer("xxx".as_bytes());
        assert!(!squash.get_squash_buffer().is_empty());

        squash.update_squash_buffer("x".repeat(123456).as_bytes());
        assert!(!squash.get_squash_buffer().is_empty());
    }
}
