use path_clean::PathClean;
use std::os::unix::fs::FileTypeExt;

#[derive(Debug)]
pub(crate) enum FileType {
    Dir,
    Reg,
    Device,
    Symlink,
    Unsupported,
    Invalid,
}

impl FileType {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            FileType::Dir => "directory",
            FileType::Reg => "regular file",
            FileType::Device => "device",
            FileType::Symlink => "symlink",
            FileType::Unsupported => "unsupported file",
            FileType::Invalid => "invalid file",
        }
    }

    pub(crate) fn is_dir(&self) -> bool {
        matches!(self, FileType::Dir)
    }

    #[allow(dead_code)]
    pub(crate) fn is_reg(&self) -> bool {
        matches!(self, FileType::Reg)
    }

    #[allow(dead_code)]
    pub(crate) fn is_device(&self) -> bool {
        matches!(self, FileType::Device)
    }

    pub(crate) fn is_symlink(&self) -> bool {
        matches!(self, FileType::Symlink)
    }

    #[allow(dead_code)]
    pub(crate) fn is_unsupported(&self) -> bool {
        matches!(self, FileType::Unsupported)
    }

    pub(crate) fn is_invalid(&self) -> bool {
        matches!(self, FileType::Invalid)
    }
}

pub(crate) fn canonicalize_path(f: &str) -> std::io::Result<String> {
    let p = match std::fs::canonicalize(f) {
        Ok(v) => v,
        Err(e) => {
            if std::fs::symlink_metadata(f)?.file_type().is_symlink() {
                return Ok(String::new()); // ignore broken symlink
            }
            return Err(e);
        }
    };
    p.into_os_string()
        .into_string()
        .map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidInput))
}

// This function
// * does not resolve symlink
// * works with non existent path
pub(crate) fn get_abspath(f: &str) -> std::io::Result<String> {
    let p = std::path::Path::new(f);
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        std::env::current_dir()?.join(f)
    }
    .clean()
    .into_os_string()
    .into_string()
    .map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidInput))
}

// fails if f is "/" or equivalent
pub(crate) fn get_dirpath(f: &str) -> std::io::Result<String> {
    Ok(std::path::Path::new(&get_abspath(f)?)
        .parent()
        .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))?
        .to_str()
        .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::InvalidInput))?
        .to_string())
}

// fails if f is "/" or equivalent
pub(crate) fn get_basename(f: &str) -> std::io::Result<String> {
    Ok(std::path::Path::new(&get_abspath(f)?)
        .file_name()
        .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))?
        .to_str()
        .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::InvalidInput))?
        .to_string())
}

pub(crate) fn is_abspath(f: &str) -> bool {
    std::path::Path::new(f).is_absolute()
}

pub(crate) fn is_windows() -> bool {
    std::env::consts::OS == "windows"
}

pub(crate) fn get_path_separator() -> char {
    std::path::MAIN_SEPARATOR
}

pub(crate) fn get_raw_file_type(f: &str) -> std::io::Result<FileType> {
    match std::fs::symlink_metadata(f) {
        Ok(v) => Ok(get_mode_type(v.file_type())),
        Err(e) => Err(e),
    }
}

pub(crate) fn get_file_type(f: &str) -> std::io::Result<FileType> {
    match std::fs::metadata(f) {
        Ok(v) => Ok(get_mode_type(v.file_type())),
        Err(e) => Err(e),
    }
}

fn get_mode_type(t: std::fs::FileType) -> FileType {
    if t.is_dir() {
        FileType::Dir
    } else if t.is_file() {
        FileType::Reg
    } else if t.is_symlink() {
        FileType::Symlink
    } else if t.is_block_device() || t.is_char_device() {
        FileType::Device
    } else {
        FileType::Unsupported
    }
}

pub(crate) fn path_exists_or_error(f: &str) -> std::io::Result<std::fs::Metadata> {
    std::fs::metadata(f)
}

#[allow(dead_code)]
pub(crate) fn path_exists(f: &str) -> bool {
    std::path::Path::new(f).exists()
}

pub(crate) fn is_valid_hexsum(s: &str) -> (&str, bool) {
    let orig = s;
    let s = match s.strip_prefix("0x") {
        Some(v) => v,
        None => s,
    };

    if s.len() < 32 {
        return (orig, false);
    }

    for r in s.chars() {
        if !r.is_ascii_digit() && !('a'..='f').contains(&r) && !('A'..='F').contains(&r) {
            return (orig, false);
        }
    }
    (s, true)
}

pub(crate) fn get_xsum_format_string(f: &str, h: &str, swap: bool) -> String {
    if !swap {
        // compatible with shaXsum commands
        format!("{h}  {f}")
    } else {
        format!("{f}  {h}")
    }
}

pub(crate) fn get_num_format_string(n: usize, msg: &str) -> String {
    if msg.is_empty() {
        return "???".to_string();
    }

    let mut s = format!("{n} {msg}");
    if n > 1 {
        if msg == FileType::Dir.as_str() {
            s = format!("{}{}", &s[..s.len() - 1], "ies");
            assert!(s.ends_with("directories"));
        } else {
            s += "s";
        }
    }
    s
}

pub(crate) fn print_num_format_string(n: usize, msg: &str) {
    println!("{}", get_num_format_string(n, msg));
}

pub(crate) fn panic_file_type(f: &str, how: &str, t: &FileType) {
    if !f.is_empty() {
        panic!("{f} has {how} file type {t:?}");
    } else {
        panic!("{how} file type {t:?}");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_canonicalize_path() {
        #[derive(Debug)]
        struct F {
            i: &'static str,
            o: &'static str,
        }
        let path_list = [
            F { i: "/", o: "/" },
            F { i: "/////", o: "/" },
            F { i: "/..", o: "/" },
            F { i: "/../", o: "/" },
            F {
                i: "/root",
                o: "/root",
            },
            F {
                i: "/root/",
                o: "/root",
            },
            F {
                i: "/root/..",
                o: "/",
            },
            F {
                i: "/root/../dev",
                o: "/dev",
            },
        ];
        for x in &path_list {
            match super::canonicalize_path(x.i) {
                Ok(v) => assert_eq!(v, x.o),
                Err(e) => panic!("{e} {x:?}"),
            }
        }
    }

    #[test]
    fn test_get_abspath() {
        #[derive(Debug)]
        struct F {
            i: &'static str,
            o: &'static str,
        }
        let path_list = [
            F { i: "/", o: "/" },
            F { i: "/////", o: "/" },
            F { i: "/..", o: "/" },
            F { i: "/../", o: "/" },
            F {
                i: "/root",
                o: "/root",
            },
            F {
                i: "/root/",
                o: "/root",
            },
            F {
                i: "/root/..",
                o: "/",
            },
            F {
                i: "/root/../dev",
                o: "/dev",
            },
            F {
                i: "/does/not/exist",
                o: "/does/not/exist",
            },
            F {
                i: "/does/not/./exist",
                o: "/does/not/exist",
            },
            F {
                i: "/does/not/../NOT/exist",
                o: "/does/NOT/exist",
            },
        ];
        for x in &path_list {
            match super::get_abspath(x.i) {
                Ok(v) => assert_eq!(v, x.o),
                Err(e) => panic!("{e} {x:?}"),
            }
        }
    }

    #[test]
    fn test_get_dirpath() {
        #[derive(Debug)]
        struct F {
            i: &'static str,
            o: &'static str,
        }
        let path_list = [
            F { i: "/root", o: "/" },
            F {
                i: "/root/",
                o: "/",
            },
            F {
                i: "/root/../dev",
                o: "/",
            },
            F {
                i: "/does/not/exist",
                o: "/does/not",
            },
            F {
                i: "/does/not/./exist",
                o: "/does/not",
            },
            F {
                i: "/does/not/../NOT/exist",
                o: "/does/NOT",
            },
        ];
        for x in &path_list {
            match super::get_dirpath(x.i) {
                Ok(v) => assert_eq!(v, x.o),
                Err(e) => panic!("{e} {x:?}"),
            }
        }
    }

    #[test]
    fn test_get_basename() {
        #[derive(Debug)]
        struct F {
            i: &'static str,
            o: &'static str,
        }
        let path_list = [
            F {
                i: "/root",
                o: "root",
            },
            F {
                i: "/root/",
                o: "root",
            },
            F {
                i: "/root/../dev",
                o: "dev",
            },
            F {
                i: "/does/not/exist",
                o: "exist",
            },
            F {
                i: "/does/not/./exist",
                o: "exist",
            },
            F {
                i: "/does/not/../NOT/exist",
                o: "exist",
            },
        ];
        for x in &path_list {
            match super::get_basename(x.i) {
                Ok(v) => assert_eq!(v, x.o),
                Err(e) => panic!("{e} {x:?}"),
            }
        }
    }

    #[test]
    fn test_is_abspath() {
        #[derive(Debug)]
        struct F {
            i: &'static str,
            o: bool,
        }
        let path_list = [
            F { i: "/", o: true },
            F {
                i: "/////",
                o: true,
            },
            F { i: "/..", o: true },
            F { i: "/../", o: true },
            F {
                i: "/root",
                o: true,
            },
            F {
                i: "/root/",
                o: true,
            },
            F {
                i: "/root/..",
                o: true,
            },
            F {
                i: "/root/../dev",
                o: true,
            },
            F {
                i: "/does/not/exist",
                o: true,
            },
            F {
                i: "/does/not/../NOT/exist",
                o: true,
            },
            F { i: "xxx", o: false },
            F {
                i: "does/not/exist",
                o: false,
            },
        ];
        for x in &path_list {
            assert_eq!(super::is_abspath(x.i), x.o, "{x:?}");
        }
    }

    #[test]
    fn test_is_windows() {
        assert!(!super::is_windows());
    }

    #[test]
    fn test_get_path_separator() {
        assert_eq!(super::get_path_separator(), '/');
    }

    #[test]
    fn test_get_raw_file_type() {
        let dir_list = [".", "..", "/", "/dev"];
        for f in &dir_list {
            match super::get_raw_file_type(f) {
                Ok(v) => match v {
                    super::FileType::Dir => (),
                    x => panic!("{x:?}"),
                },
                Err(e) => panic!("{e}"),
            }
        }
        let invalid_list = ["", "516e7cb4-6ecf-11d6-8ff8-00022d09712b"];
        for f in &invalid_list {
            if let Ok(v) = super::get_raw_file_type(f) {
                panic!("{v:?}");
            }
        }
    }

    #[test]
    fn test_get_file_type() {
        let dir_list = [".", "..", "/", "/dev"];
        for f in &dir_list {
            match super::get_file_type(f) {
                Ok(v) => match v {
                    super::FileType::Dir => (),
                    x => panic!("{x:?}"),
                },
                Err(e) => panic!("{e}"),
            }
        }
        let invalid_list = ["", "516e7cb4-6ecf-11d6-8ff8-00022d09712b"];
        for f in &invalid_list {
            if let Ok(v) = super::get_file_type(f) {
                panic!("{v:?}");
            }
        }
    }

    #[test]
    fn test_get_file_type_string() {
        struct F {
            t: super::FileType,
            s: &'static str,
        }
        let file_type_list = [
            F {
                t: super::FileType::Dir,
                s: "directory",
            },
            F {
                t: super::FileType::Reg,
                s: "regular file",
            },
            F {
                t: super::FileType::Device,
                s: "device",
            },
            F {
                t: super::FileType::Symlink,
                s: "symlink",
            },
            F {
                t: super::FileType::Unsupported,
                s: "unsupported file",
            },
            F {
                t: super::FileType::Invalid,
                s: "invalid file",
            },
        ];
        for x in &file_type_list {
            assert_eq!(x.t.as_str(), x.s);
        }
    }

    #[test]
    fn test_get_file_type_is_xxx() {
        assert!(super::FileType::Dir.is_dir());
        assert!(super::FileType::Reg.is_reg());
        assert!(super::FileType::Device.is_device());
        assert!(super::FileType::Symlink.is_symlink());
        assert!(super::FileType::Unsupported.is_unsupported());
        assert!(super::FileType::Invalid.is_invalid());
    }

    #[test]
    fn test_path_exists_or_error() {
        let dir_list = [".", "..", "/", "/dev"];
        for f in &dir_list {
            if let Err(e) = super::path_exists_or_error(f) {
                panic!("{e}");
            }
        }
        let invalid_list = ["", "516e7cb4-6ecf-11d6-8ff8-00022d09712b"];
        for f in &invalid_list {
            assert!(super::path_exists_or_error(f).is_err(), "{f}");
        }
    }

    #[test]
    fn test_path_exists() {
        let dir_list = [".", "..", "/", "/dev"];
        for f in &dir_list {
            assert!(super::path_exists(f), "{f}");
        }
        let invalid_list = ["", "516e7cb4-6ecf-11d6-8ff8-00022d09712b"];
        for f in &invalid_list {
            assert!(!super::path_exists(f), "{f}");
        }
    }

    #[test]
    fn test_is_valid_hexsum() {
        let valid_list = [
            "00000000000000000000000000000000",
            "11111111111111111111111111111111",
            "22222222222222222222222222222222",
            "33333333333333333333333333333333",
            "44444444444444444444444444444444",
            "55555555555555555555555555555555",
            "66666666666666666666666666666666",
            "77777777777777777777777777777777",
            "88888888888888888888888888888888",
            "99999999999999999999999999999999",
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
            "BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB",
            "CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC",
            "DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD",
            "EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            "cccccccccccccccccccccccccccccccc",
            "dddddddddddddddddddddddddddddddd",
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
            "ffffffffffffffffffffffffffffffff",
            "0123456789ABCDEFabcdef0123456789ABCDEFabcdef",
            "0x00000000000000000000000000000000",
            "0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
            "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "0x0123456789ABCDEFabcdef0123456789ABCDEFabcdef",
        ];
        for s in &valid_list {
            assert!(super::is_valid_hexsum(s).1);
        }

        let invalid_list = [
            "gggggggggggggggggggggggggggggggg",
            "GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG",
            "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz",
            "ZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ",
            "                                ",
            "################################",
            "--------------------------------",
            "................................",
            "@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
            "________________________________",
            "0000000000000000000000000000000",
            "0x0000000000000000000000000000000",
            "0x",
            "0",
            "",
        ];
        for s in &invalid_list {
            assert!(!super::is_valid_hexsum(s).1);
        }
    }

    #[test]
    fn test_get_num_format_string() {
        struct F {
            n: usize,
            msg: &'static str,
            result: &'static str,
        }
        let num_format_list = [
            F {
                n: 0,
                msg: "",
                result: "???",
            },
            F {
                n: 1,
                msg: "",
                result: "???",
            },
            F {
                n: 2,
                msg: "",
                result: "???",
            },
            F {
                n: 0,
                msg: "file",
                result: "0 file",
            },
            F {
                n: 1,
                msg: "file",
                result: "1 file",
            },
            F {
                n: 2,
                msg: "file",
                result: "2 files",
            },
        ];
        for x in &num_format_list {
            assert_eq!(super::get_num_format_string(x.n, x.msg), x.result);
        }
    }
}
