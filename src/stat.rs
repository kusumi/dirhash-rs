use crate::dir;
use crate::util;
use crate::Opt;

#[derive(Debug, Default)]
pub(crate) struct Stat {
    stat_directory: Vec<String>, // hashed
    stat_regular: Vec<String>,   // hashed
    stat_device: Vec<String>,    // hashed
    stat_symlink: Vec<String>,   // hashed
    stat_unsupported: Vec<String>,
    stat_invalid: Vec<String>,
    stat_ignored: Vec<String>,

    written_directory: usize, // hashed
    written_regular: usize,   // hashed
    written_device: usize,    // hashed
    written_symlink: usize,   // hashed
}

impl Stat {
    pub(crate) fn new() -> Self {
        let mut stat = Self {
            ..Default::default()
        };
        stat.init_stat();
        stat
    }

    pub(crate) fn init_stat(&mut self) {
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
    pub(crate) fn num_stat_total(&self) -> usize {
        self.num_stat_directory()
            + self.num_stat_regular()
            + self.num_stat_device()
            + self.num_stat_symlink()
    }

    pub(crate) fn num_stat_directory(&self) -> usize {
        self.stat_directory.len()
    }

    pub(crate) fn num_stat_regular(&self) -> usize {
        self.stat_regular.len()
    }

    pub(crate) fn num_stat_device(&self) -> usize {
        self.stat_device.len()
    }

    pub(crate) fn num_stat_symlink(&self) -> usize {
        self.stat_symlink.len()
    }

    #[allow(dead_code)]
    pub(crate) fn num_stat_unsupported(&self) -> usize {
        self.stat_unsupported.len()
    }

    #[allow(dead_code)]
    pub(crate) fn num_stat_invalid(&self) -> usize {
        self.stat_invalid.len()
    }

    #[allow(dead_code)]
    pub(crate) fn num_stat_ignored(&self) -> usize {
        self.stat_ignored.len()
    }

    // append stat
    pub(crate) fn append_stat_total(&self) {}

    pub(crate) fn append_stat_directory(&mut self, f: &str) {
        self.stat_directory.push(f.to_string());
    }

    pub(crate) fn append_stat_regular(&mut self, f: &str) {
        self.stat_regular.push(f.to_string());
    }

    pub(crate) fn append_stat_device(&mut self, f: &str) {
        self.stat_device.push(f.to_string());
    }

    pub(crate) fn append_stat_symlink(&mut self, f: &str) {
        self.stat_symlink.push(f.to_string());
    }

    pub(crate) fn append_stat_unsupported(&mut self, f: &str) {
        self.stat_unsupported.push(f.to_string());
    }

    pub(crate) fn append_stat_invalid(&mut self, f: &str) {
        self.stat_invalid.push(f.to_string());
    }

    pub(crate) fn append_stat_ignored(&mut self, f: &str) {
        self.stat_ignored.push(f.to_string());
    }

    // print stat
    #[allow(dead_code)]
    pub(crate) fn print_stat_directory(&self, inp: &str, opt: &Opt) -> std::io::Result<()> {
        self.print_stat(&self.stat_directory, util::FileType::Dir.as_str(), inp, opt)
    }

    #[allow(dead_code)]
    pub(crate) fn print_stat_regular(&self, inp: &str, opt: &Opt) -> std::io::Result<()> {
        self.print_stat(&self.stat_regular, util::FileType::Reg.as_str(), inp, opt)
    }

    #[allow(dead_code)]
    pub(crate) fn print_stat_device(&self, inp: &str, opt: &Opt) -> std::io::Result<()> {
        self.print_stat(&self.stat_device, util::FileType::Device.as_str(), inp, opt)
    }

    #[allow(dead_code)]
    pub(crate) fn print_stat_symlink(&self, inp: &str, opt: &Opt) -> std::io::Result<()> {
        self.print_stat(
            &self.stat_symlink,
            util::FileType::Symlink.as_str(),
            inp,
            opt,
        )
    }

    pub(crate) fn print_stat_unsupported(&self, inp: &str, opt: &Opt) -> std::io::Result<()> {
        self.print_stat(
            &self.stat_unsupported,
            util::FileType::Unsupported.as_str(),
            inp,
            opt,
        )
    }

    pub(crate) fn print_stat_invalid(&self, inp: &str, opt: &Opt) -> std::io::Result<()> {
        self.print_stat(
            &self.stat_invalid,
            util::FileType::Invalid.as_str(),
            inp,
            opt,
        )
    }

    pub(crate) fn print_stat_ignored(&self, inp: &str, opt: &Opt) -> std::io::Result<()> {
        self.print_stat(&self.stat_ignored, "ignored file", inp, opt)
    }

    fn print_stat(&self, l: &[String], msg: &str, inp: &str, opt: &Opt) -> std::io::Result<()> {
        if l.is_empty() {
            return Ok(());
        }
        util::print_num_format_string(l.len(), msg);

        for v in l {
            let f = dir::get_real_path(v, inp, opt);
            let t1 = util::get_raw_file_type(v)?;
            let t2 = match util::get_file_type(v) {
                Ok(v) => v,
                Err(_) => util::FileType::Invalid, // e.g. broken symlink
            };
            assert!(!t2.is_symlink()); // symlink chains resolved
            if t1.is_symlink() {
                assert!(opt.ignore_symlink || t2.is_dir() || t2.is_invalid());
                println!("{} ({} -> {})", f, t1.as_str(), t2.as_str());
            } else {
                assert!(!t2.is_dir());
                println!("{} ({})", f, t1.as_str());
            }
        }
        Ok(())
    }

    // num written
    pub(crate) fn num_written_total(&self) -> usize {
        self.num_written_directory()
            + self.num_written_regular()
            + self.num_written_device()
            + self.num_written_symlink()
    }

    pub(crate) fn num_written_directory(&self) -> usize {
        self.written_directory
    }

    pub(crate) fn num_written_regular(&self) -> usize {
        self.written_regular
    }

    pub(crate) fn num_written_device(&self) -> usize {
        self.written_device
    }

    pub(crate) fn num_written_symlink(&self) -> usize {
        self.written_symlink
    }

    // append written
    pub(crate) fn append_written_total(&self, _written: u64) {}

    pub(crate) fn append_written_directory(&mut self, written: u64) {
        self.written_directory += usize::try_from(written).unwrap();
    }

    pub(crate) fn append_written_regular(&mut self, written: u64) {
        self.written_regular += usize::try_from(written).unwrap();
    }

    pub(crate) fn append_written_device(&mut self, written: u64) {
        self.written_device += usize::try_from(written).unwrap();
    }

    pub(crate) fn append_written_symlink(&mut self, written: u64) {
        self.written_symlink += usize::try_from(written).unwrap();
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
        stat.append_written_regular(9_999_999_999);
        assert_eq!(stat.num_written_regular(), 9_999_999_999);

        stat.append_written_regular(1);
        assert_eq!(stat.num_written_regular(), 10_000_000_000);

        stat.init_stat();
        assert_eq!(stat.num_written_regular(), 0);
    }
}
