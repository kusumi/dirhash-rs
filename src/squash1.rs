#[cfg(feature = "squash1")]
use crate::hash;

#[derive(Debug, Default)]
pub struct Squash {
    pub squash_buffer: Vec<Vec<u8>>,
}

impl Squash {
    pub fn init_squash_buffer(&mut self) {
        self.squash_buffer.clear();
    }

    pub fn update_squash_buffer(&mut self, b: &[u8]) {
        match hash::get_byte_hash(b, hash::MD5) {
            Ok(v) => {
                let hash::HashValue { b, .. } = v;
                self.squash_buffer.push(b);
            }
            Err(e) => panic!("{}", e),
        }
    }

    pub fn get_squash_buffer(&self) -> Vec<u8> {
        // XXX directly sort Vec<Vec<u8>>
        let mut s = Vec::new();
        for v in self.squash_buffer.iter() {
            s.push(hash::get_hex_sum(v));
        }

        s.sort();
        return s.join("").as_bytes().to_vec();
    }
}
