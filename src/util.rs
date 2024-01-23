use std::os::unix::fs::FileTypeExt;

pub type FileType = i32;

pub const DIR: FileType = 0;
pub const REG: FileType = 1;
pub const DEVICE: FileType = 2;
pub const SYMLINK: FileType = 3;
pub const UNSUPPORTED: FileType = 4;
pub const INVALID: FileType = 5;

pub const DIR_STR: &str = "directory";
pub const REG_STR: &str = "regular file";
pub const DEVICE_STR: &str = "device";
pub const SYMLINK_STR: &str = "symlink";
pub const UNSUPPORTED_STR: &str = "unsupported file";
pub const INVALID_STR: &str = "invalid file";

pub fn read_link(f: &str) -> std::io::Result<String> {
    let p = std::fs::read_link(f)?;
    Ok(p.into_os_string().into_string().unwrap())
}

pub fn canonicalize_path(f: &str) -> std::io::Result<String> {
    let p = match std::fs::canonicalize(f) {
        Ok(v) => v,
        Err(e) => {
            if std::fs::symlink_metadata(f)?.file_type().is_symlink() {
                return Ok("".to_string()); // ignore broken symlink
            } else {
                return Err(e);
            }
        }
    };
    Ok(p.into_os_string().into_string().unwrap())
}

pub fn get_abspath(f: &str) -> std::io::Result<String> {
    let p = std::fs::canonicalize(f)?; // XXX keep symlink unresolved
    Ok(p.into_os_string().into_string().unwrap())
}

pub fn get_dirpath(f: &str) -> std::io::Result<String> {
    let p = std::path::Path::new(f)
        .parent()
        .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))?;
    Ok(p.to_str().unwrap().to_string())
}

pub fn get_basename(f: &str) -> std::io::Result<String> {
    let s = std::path::Path::new(f)
        .file_name()
        .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))?;
    Ok(s.to_str().unwrap().to_string())
}

pub fn is_abspath(f: &str) -> bool {
    //f == get_abspath(f).unwrap() // XXX doesn't work with symlink
    &f[0..1] == "/"
}

pub fn is_windows() -> bool {
    std::env::consts::OS == "windows"
}

pub fn get_path_separator() -> char {
    std::path::MAIN_SEPARATOR
}

pub fn get_raw_file_type(f: &str) -> std::io::Result<FileType> {
    match std::fs::symlink_metadata(f) {
        Ok(v) => Ok(get_mode_type(&v.file_type())),
        Err(e) => Err(e),
    }
}

pub fn get_file_type(f: &str) -> std::io::Result<FileType> {
    match std::fs::metadata(f) {
        Ok(v) => Ok(get_mode_type(&v.file_type())),
        Err(e) => Err(e),
    }
}

pub fn get_file_type_string(t: FileType) -> &'static str {
    match t {
        DIR => DIR_STR,
        REG => REG_STR,
        DEVICE => DEVICE_STR,
        SYMLINK => SYMLINK_STR,
        UNSUPPORTED => UNSUPPORTED_STR,
        INVALID => INVALID_STR,
        _ => {
            panic_file_type("", "unknown", t);
            ""
        }
    }
}

pub fn get_mode_type(t: &std::fs::FileType) -> FileType {
    if t.is_dir() {
        DIR
    } else if t.is_file() {
        REG
    } else if t.is_symlink() {
        SYMLINK
    } else if t.is_block_device() || t.is_char_device() {
        DEVICE
    } else {
        UNSUPPORTED
    }
}

pub fn path_exists(f: &str) -> std::io::Result<std::fs::Metadata> {
    std::fs::metadata(f)
}

pub fn is_valid_hexsum(s: &str) -> (&str, bool) {
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

pub fn get_xsum_format_string(f: &str, h: &str) -> String {
    format!("{}  {}", h, f)
}

pub fn get_num_format_string(n: usize, msg: &str) -> String {
    if msg.is_empty() {
        return "???".to_string();
    }

    let mut s = format!("{} {}", n, msg);
    if n > 1 {
        if msg == DIR_STR {
            s = format!("{}{}", &s[..s.len() - 1], "ies");
            assert!(s.ends_with("directories"));
        } else {
            s += "s";
        }
    }
    s
}

pub fn print_num_format_string(n: usize, msg: &str) {
    println!("{}", get_num_format_string(n, msg));
}

pub fn panic_file_type(f: &str, how: &str, t: FileType) {
    if !f.is_empty() {
        panic!("{} has {} file type {}", f, how, t);
    } else {
        panic!("{} file type {}", how, t);
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
        for x in path_list.iter() {
            match super::canonicalize_path(x.i) {
                Ok(v) => assert_eq!(v, x.o),
                Err(e) => panic!("{} {:?}", e, x),
            }
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
        for f in dir_list.iter() {
            match super::get_raw_file_type(f) {
                Ok(v) => match v {
                    super::DIR => (),
                    x => panic!("{}", x),
                },
                Err(e) => panic!("{}", e),
            }
        }
        let invalid_list = ["", "516e7cb4-6ecf-11d6-8ff8-00022d09712b"];
        for f in invalid_list.iter() {
            if let Ok(v) = super::get_raw_file_type(f) {
                panic!("{}", v);
            }
        }
    }

    #[test]
    fn test_get_file_type() {
        let dir_list = [".", "..", "/", "/dev"];
        for f in dir_list.iter() {
            match super::get_file_type(f) {
                Ok(v) => match v {
                    super::DIR => (),
                    x => panic!("{}", x),
                },
                Err(e) => panic!("{}", e),
            }
        }
        let invalid_list = ["", "516e7cb4-6ecf-11d6-8ff8-00022d09712b"];
        for f in invalid_list.iter() {
            if let Ok(v) = super::get_file_type(f) {
                panic!("{}", v);
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
                t: super::DIR,
                s: "directory",
            },
            F {
                t: super::REG,
                s: "regular file",
            },
            F {
                t: super::DEVICE,
                s: "device",
            },
            F {
                t: super::SYMLINK,
                s: "symlink",
            },
            F {
                t: super::UNSUPPORTED,
                s: "unsupported file",
            },
            F {
                t: super::INVALID,
                s: "invalid file",
            },
        ];
        for x in file_type_list.iter() {
            assert_eq!(super::get_file_type_string(x.t), x.s);
        }
    }

    #[test]
    fn test_path_exists() {
        let dir_list = [".", "..", "/", "/dev"];
        for f in dir_list.iter() {
            if let Err(e) = super::path_exists(f) {
                panic!("{}", e);
            }
        }
        let invalid_list = ["", "516e7cb4-6ecf-11d6-8ff8-00022d09712b"];
        for f in invalid_list.iter() {
            if super::path_exists(f).is_ok() {
                panic!("{}", f);
            }
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
        for s in valid_list.iter() {
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
        for s in invalid_list.iter() {
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
        for x in num_format_list.iter() {
            assert_eq!(super::get_num_format_string(x.n, x.msg), x.result);
        }
    }
}
