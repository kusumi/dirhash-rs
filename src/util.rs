use std::os::unix::fs::FileTypeExt;

// XXX Rust has std::fs::FileType
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

pub fn read_link(f: &str) -> Result<String, std::io::Error> {
    let p = std::fs::read_link(f)?;
    Ok(p.into_os_string().into_string().unwrap())
}

pub fn get_abspath(f: &str) -> Result<String, std::io::Error> {
    let p = std::fs::canonicalize(f)?; // XXX keep symlink unresolved
    Ok(p.into_os_string().into_string().unwrap())
}

pub fn get_dirpath(f: &str) -> Result<String, std::io::Error> {
    let p = std::path::Path::new(f);
    let path = p.parent().unwrap();
    return Ok(path.to_str().unwrap().to_string());
}

pub fn get_basename(f: &str) -> Result<String, std::io::Error> {
    let p = std::path::Path::new(f);
    let path = p.file_name().unwrap();
    return Ok(path.to_str().unwrap().to_string());
}

pub fn is_abspath(f: &str) -> bool {
    //f == get_abspath(f).unwrap() // XXX doesn't work with symlink
    &f[0..1] == "/"
}

// XXX behaves differently from filepath.Join which resolves ".." entries
pub fn join_path(f1: &str, f2: &str) -> String {
    let p = std::path::Path::new(f1);
    p.join(f2).as_path().to_str().unwrap().to_string()
}

pub fn is_windows() -> bool {
    std::env::consts::OS == "windows"
}

pub fn get_path_separator() -> String {
    std::path::MAIN_SEPARATOR.to_string()
}

pub fn get_raw_file_type(f: &str) -> Result<FileType, std::io::Error> {
    let m = std::fs::symlink_metadata(f)?;
    Ok(get_mode_type(&m.file_type()))
}

pub fn get_file_type(f: &str) -> Result<FileType, std::io::Error> {
    let m = std::fs::metadata(f)?;
    Ok(get_mode_type(&m.file_type()))
}

pub fn get_file_type_string(t: FileType) -> String {
    return match t {
        DIR => DIR_STR,
        REG => REG_STR,
        DEVICE => DEVICE_STR,
        SYMLINK => SYMLINK_STR,
        UNSUPPORTED => UNSUPPORTED_STR,
        INVALID => INVALID_STR,
        _ => panic!("Unknown file type {}", t),
    }
    .to_string();
}

pub fn get_mode_type(t: &std::fs::FileType) -> FileType {
    if t.is_dir() {
        return DIR;
    } else if t.is_file() {
        return REG;
    } else if t.is_symlink() {
        return SYMLINK;
    } else if t.is_block_device() || t.is_char_device() {
        return DEVICE;
    }
    UNSUPPORTED
}

pub fn path_exists(f: &str) -> Result<(), std::io::Error> {
    std::fs::metadata(f)?;
    Ok(())
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
        if !('0'..='9').contains(&r) && !('a'..='f').contains(&r) && !('A'..='F').contains(&r) {
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
        s += "s";
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
