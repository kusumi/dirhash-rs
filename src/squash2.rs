#[cfg(feature = "squash2")]
use crate::hash;

#[derive(Debug, Default)]
pub struct Squash {
    pub squash_buffer: Vec<u8>,
}

impl Squash {
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
