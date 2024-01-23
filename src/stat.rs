use crate::dir;
use crate::util;
use crate::Opt;

#[derive(Debug, Default)]
pub struct Stat {
    stat_directory: Vec<String>, // hashed
    stat_regular: Vec<String>,   // hashed
    stat_device: Vec<String>,    // hashed
    stat_symlink: Vec<String>,   // hashed
    stat_unsupported: Vec<String>,
    stat_invalid: Vec<String>,
    stat_ignored: Vec<String>,

    written_directory: u64, // hashed
    written_regular: u64,   // hashed
    written_device: u64,    // hashed
    written_symlink: u64,   // hashed
}

impl Stat {
    pub fn new() -> Stat {
        let mut stat = Stat { ..Stat::default() };
        stat.init_stat();
        stat
    }

    pub fn init_stat(&mut self) {
        self.stat_directory.clear();
        self.stat_regular.clear();
        self.stat_device.clear();
        self.stat_symlink.clear();
        self.stat_unsupported.clear();
        self.stat_invalid.clear();
        self.stat_ignored.clear();

        self.written_directory = 0;
        self.written_regular = 0;
        self.written_device = 0;
        self.written_symlink = 0;
    }

    // num stat
    pub fn num_stat_total(&self) -> u64 {
        self.num_stat_directory()
            + self.num_stat_regular()
            + self.num_stat_device()
            + self.num_stat_symlink()
    }

    pub fn num_stat_directory(&self) -> u64 {
        self.stat_directory.len() as u64
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

    #[allow(dead_code)]
    pub fn num_stat_unsupported(&self) -> u64 {
        self.stat_unsupported.len() as u64
    }

    #[allow(dead_code)]
    pub fn num_stat_invalid(&self) -> u64 {
        self.stat_invalid.len() as u64
    }

    #[allow(dead_code)]
    pub fn num_stat_ignored(&self) -> u64 {
        self.stat_ignored.len() as u64
    }

    // append stat
    pub fn append_stat_total(&mut self) {}

    pub fn append_stat_directory(&mut self, f: &str) {
        self.stat_directory.push(f.to_string());
    }

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
    #[allow(dead_code)]
    pub fn print_stat_directory(&self, inp: &str, opt: &Opt) -> std::io::Result<()> {
        self.print_stat(&self.stat_directory, util::DIR_STR, inp, opt)
    }

    #[allow(dead_code)]
    pub fn print_stat_regular(&self, inp: &str, opt: &Opt) -> std::io::Result<()> {
        self.print_stat(&self.stat_regular, util::REG_STR, inp, opt)
    }

    #[allow(dead_code)]
    pub fn print_stat_device(&self, inp: &str, opt: &Opt) -> std::io::Result<()> {
        self.print_stat(&self.stat_device, util::DEVICE_STR, inp, opt)
    }

    #[allow(dead_code)]
    pub fn print_stat_symlink(&self, inp: &str, opt: &Opt) -> std::io::Result<()> {
        self.print_stat(&self.stat_symlink, util::SYMLINK_STR, inp, opt)
    }

    pub fn print_stat_unsupported(&self, inp: &str, opt: &Opt) -> std::io::Result<()> {
        self.print_stat(&self.stat_unsupported, util::UNSUPPORTED_STR, inp, opt)
    }

    pub fn print_stat_invalid(&self, inp: &str, opt: &Opt) -> std::io::Result<()> {
        self.print_stat(&self.stat_invalid, util::INVALID_STR, inp, opt)
    }

    pub fn print_stat_ignored(&self, inp: &str, opt: &Opt) -> std::io::Result<()> {
        self.print_stat(&self.stat_ignored, "ignored file", inp, opt)
    }

    fn print_stat(&self, l: &Vec<String>, msg: &str, inp: &str, opt: &Opt) -> std::io::Result<()> {
        if l.is_empty() {
            return Ok(());
        }
        util::print_num_format_string(l.len(), msg);

        for v in l.iter() {
            let f = dir::get_real_path(v, inp, opt);
            let t1 = util::get_raw_file_type(v)?;
            let t2 = match util::get_file_type(v) {
                Ok(v) => v,
                Err(_) => util::INVALID, // e.g. broken symlink
            };
            assert!(t2 != util::SYMLINK); // symlink chains resolved
            if t1 == util::SYMLINK {
                assert!(opt.ignore_symlink || t2 == util::DIR || t2 == util::INVALID);
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
        Ok(())
    }

    // num written
    pub fn num_written_total(&self) -> u64 {
        self.num_written_directory()
            + self.num_written_regular()
            + self.num_written_device()
            + self.num_written_symlink()
    }

    pub fn num_written_directory(&self) -> u64 {
        self.written_directory
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

    pub fn append_written_directory(&mut self, written: u64) {
        self.written_directory += written;
    }

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

#[cfg(test)]
mod tests {
    #[test]
    fn test_num_stat_regular() {
        // 0
        let mut stat = super::Stat::new();
        assert_eq!(stat.num_stat_regular(), 0);

        // 0
        stat.init_stat();
        assert_eq!(stat.num_stat_regular(), 0);
    }

    #[test]
    fn test_append_stat_regular() {
        // 1
        let mut stat = super::Stat::new();
        stat.append_stat_regular("a");
        assert_eq!(stat.num_stat_regular(), 1);
        assert_eq!(stat.stat_regular[0], "a");

        // 2
        stat.append_stat_regular("b");
        assert_eq!(stat.num_stat_regular(), 2);
        assert_eq!(stat.stat_regular[0], "a");
        assert_eq!(stat.stat_regular[1], "b");

        // 3
        stat.append_stat_regular("c");
        assert_eq!(stat.num_stat_regular(), 3);
        assert_eq!(stat.stat_regular[0], "a");
        assert_eq!(stat.stat_regular[1], "b");
        assert_eq!(stat.stat_regular[2], "c");

        // 1
        stat.init_stat();
        stat.append_stat_regular("d");
        assert_eq!(stat.num_stat_regular(), 1);
        assert_eq!(stat.stat_regular[0], "d");
    }

    #[test]
    fn test_num_written_regular() {
        let mut stat = super::Stat::new();
        assert_eq!(stat.num_written_regular(), 0);

        stat.init_stat();
        assert_eq!(stat.num_written_regular(), 0);
    }

    #[test]
    fn test_append_written_regular() {
        let mut stat = super::Stat::new();
        stat.append_written_regular(9999999999);
        assert_eq!(stat.num_written_regular(), 9999999999);

        stat.append_written_regular(1);
        assert_eq!(stat.num_written_regular(), 10000000000);

        stat.init_stat();
        assert_eq!(stat.num_written_regular(), 0);
    }
}
