use crate::dir;
use crate::util;

#[derive(Debug, Default)]
pub struct Stat {
    pub stat_regular: Vec<String>, // hashed
    pub stat_device: Vec<String>,  // hashed
    pub stat_symlink: Vec<String>, // hashed
    pub stat_unsupported: Vec<String>,
    pub stat_invalid: Vec<String>,
    pub stat_ignored: Vec<String>,

    pub written_regular: u64, // hashed
    pub written_device: u64,  // hashed
    pub written_symlink: u64, // hashed
}

impl Stat {
    pub fn init_stat(&mut self) {
        self.stat_regular.clear();
        self.stat_device.clear();
        self.stat_symlink.clear();
        self.stat_unsupported.clear();
        self.stat_invalid.clear();
        self.stat_ignored.clear();

        self.written_regular = 0;
        self.written_device = 0;
        self.written_symlink = 0;
    }

    // num stat
    pub fn num_stat_total(&self) -> u64 {
        self.num_stat_regular() + self.num_stat_device() + self.num_stat_symlink()
    }

    pub fn num_stat_regular(&self) -> u64 {
        self.stat_regular.len() as u64
    }

    pub fn num_stat_device(&self) -> u64 {
        self.stat_device.len() as u64
    }

    pub fn num_stat_symlink(&self) -> u64 {
        self.stat_symlink.len() as u64
    }

    /*
    pub fn num_stat_unsupported(&self) -> u64 {
        self.stat_unsupported.len() as u64
    }

    pub fn num_stat_invalid(&self) -> u64 {
        self.stat_invalid.len() as u64
    }

    pub fn num_stat_ignored(&self) -> u64 {
        self.stat_ignored.len() as u64
    }
    */

    // append stat
    pub fn append_stat_total(&mut self) {}

    pub fn append_stat_regular(&mut self, f: &str) {
        self.stat_regular.push(f.to_string());
    }

    pub fn append_stat_device(&mut self, f: &str) {
        self.stat_device.push(f.to_string());
    }

    pub fn append_stat_symlink(&mut self, f: &str) {
        self.stat_symlink.push(f.to_string());
    }

    pub fn append_stat_unsupported(&mut self, f: &str) {
        self.stat_unsupported.push(f.to_string());
    }

    pub fn append_stat_invalid(&mut self, f: &str) {
        self.stat_invalid.push(f.to_string());
    }

    pub fn append_stat_ignored(&mut self, f: &str) {
        self.stat_ignored.push(f.to_string());
    }

    // print stat
    /*
    pub fn print_stat_regular(&self, dat: &crate::UserData) {
        self.print_stat(&self.stat_regular, util::REG_STR, dat);
    }

    pub fn print_stat_device(&self, dat: &crate::UserData) {
        self.print_stat(&self.stat_device, util::DEVICE_STR, dat);
    }

    pub fn print_stat_symlink(&self, dat: &crate::UserData) {
        self.print_stat(&self.stat_symlink, util::SYMLINK_STR, dat);
    }
    */

    pub fn print_stat_unsupported(&self, dat: &crate::UserData) {
        self.print_stat(&self.stat_unsupported, util::UNSUPPORTED_STR, dat);
    }

    pub fn print_stat_invalid(&self, dat: &crate::UserData) {
        self.print_stat(&self.stat_invalid, util::INVALID_STR, dat);
    }

    pub fn print_stat_ignored(&self, dat: &crate::UserData) {
        self.print_stat(&self.stat_ignored, "ignored file", dat);
    }

    fn print_stat(&self, l: &Vec<String>, msg: &str, dat: &crate::UserData) {
        if l.is_empty() {
            return;
        }
        util::print_num_format_string(l.len(), msg);

        for v in l.iter() {
            let f = dir::get_real_path(v, dat);
            let t1 = match util::get_raw_file_type(v) {
                Ok(t) => t,
                Err(e) => panic!("{}", e),
            };
            let t2 = match util::get_file_type(v) {
                Ok(t) => t,
                Err(e) => panic!("{}", e),
            };
            assert!(t2 != util::SYMLINK); // symlink chains resolved
            if t1 == util::SYMLINK {
                assert!(dat.opt.ignore_symlink || t2 == util::DIR);
                println!(
                    "{} ({} -> {})",
                    f,
                    util::get_file_type_string(t1),
                    util::get_file_type_string(t2)
                );
            } else {
                assert!(t2 != util::DIR);
                println!("{} ({})", f, util::get_file_type_string(t1));
            }
        }
    }

    // num written
    pub fn num_written_total(&self) -> u64 {
        self.num_written_regular() + self.num_written_device() + self.num_written_symlink()
    }

    pub fn num_written_regular(&self) -> u64 {
        self.written_regular
    }

    pub fn num_written_device(&self) -> u64 {
        self.written_device
    }

    pub fn num_written_symlink(&self) -> u64 {
        self.written_symlink
    }

    // append written
    pub fn append_written_total(&mut self, _written: u64) {}

    pub fn append_written_regular(&mut self, written: u64) {
        self.written_regular += written;
    }

    pub fn append_written_device(&mut self, written: u64) {
        self.written_device += written;
    }

    pub fn append_written_symlink(&mut self, written: u64) {
        self.written_symlink += written;
    }
}
