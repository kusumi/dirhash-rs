use crate::hash;
use crate::util;

impl crate::UserData {
    fn get_input_prefix(&self) -> &str {
        &self.input_prefix
    }
}

pub fn print_input(f: &str, dat: &mut crate::UserData) -> Result<(), std::io::Error> {
    if dat.opt.debug {
        println!("{}: {}", stringify!(print_input), f);
    }

    // assert exists
    util::path_exists(f)?;

    // convert input to abs first
    let f = util::get_abspath(f)?;
    assert_file_path(&f, dat);

    // keep input prefix based on raw type
    match util::get_file_type(&f) {
        util::DIR => dat.input_prefix = f.clone(),
        util::REG | util::DEVICE => dat.input_prefix = util::get_dirpath(&f)?,
        _ => return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput)),
    }

    // prefix is a directory
    assert!(util::get_file_type(dat.get_input_prefix()) == util::DIR);

    // initialize global resource
    dat.stat.init_stat();
    dat.squash.init_squash_buffer();

    // start directory walk
    walk_directory(&f, dat)?;

    // print various stats
    if dat.opt.verbose {
        print_verbose_stat(dat);
    }
    dat.stat.print_stat_unsupported(dat);
    dat.stat.print_stat_invalid(dat);

    // print squash hash if specified
    if dat.opt.squash {
        let b = dat.squash.get_squash_buffer();
        if dat.opt.verbose {
            util::print_num_format_string(b.len(), "squashed byte");
        }
        print_byte(&f, &b, dat)?;
    }

    Ok(())
}

/*
 * walkdir::WalkDir has different traversal order vs filepath.WalkDir,
 * hence squash2 hash won't match the original golang implementation.
 */
fn walk_directory(dirpath: &str, dat: &mut crate::UserData) -> Result<(), std::io::Error> {
    for entry in walkdir::WalkDir::new(dirpath)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let mut f = match entry.path().to_str() {
            Some(v) => v,
            None => return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput)),
        };
        let mut t = util::get_raw_file_type(f);

        if test_ignore_entry(f, t, dat) {
            dat.stat.append_stat_ignored(f);
            continue;
        }

        // find target if symlink
        let mut x = String::from(f);
        let l: String; // symlink itself, not its target
        match t {
            util::SYMLINK => {
                if dat.opt.ignore_symlink {
                    dat.stat.append_stat_ignored(f);
                    continue;
                }
                if dat.opt.lstat {
                    if let Err(e) = print_symlink(f, dat) {
                        panic!("{}", e);
                    }
                    continue;
                }
                l = String::from(f);
                let tmp = util::read_link(f)?; // need tmp variable here
                f = tmp.as_str();
                if !util::is_abspath(f) {
                    let d = util::get_dirpath(&l)?;
                    x = util::join_path(&d, f).clone();
                    assert!(util::is_abspath(&x));
                } else {
                    x = String::from(f);
                }
                t = util::get_file_type(&x);
                assert!(t != util::SYMLINK); // symlink chains resolved
            }
            _ => l = String::from(""),
        }

        let res = match t {
            /*
             * A regular directory isn't considered ignored,
             * then don't count symlink to directory as ignored.
             */
            util::DIR => Ok(()),
            util::REG | util::DEVICE => print_file(&x, &l, t, dat),
            util::UNSUPPORTED => print_unsupported(&x, dat),
            util::INVALID => print_invalid(&x, dat),
            _ => {
                util::panic_file_type(&x, "unknown", t);
                Ok(())
            }
        };
        if let Err(e) = res {
            panic!("{}", e);
        }
    }

    Ok(())
}

fn test_ignore_entry(f: &str, t: util::FileType, dat: &crate::UserData) -> bool {
    assert!(util::is_abspath(f));

    // only non directory types count
    if t == util::DIR {
        return false;
    }

    let base_starts_with_dot = match util::get_basename(f) {
        Ok(v) => v.starts_with('.'),
        Err(_) => false,
    };
    let path_contains_slash_dot = f.contains("/.");

    // ignore . directories if specified
    if dat.opt.ignore_dot_dir && !base_starts_with_dot && path_contains_slash_dot {
        return true;
    }

    // ignore . regular files if specified
    if dat.opt.ignore_dot_file {
        // XXX limit to REG ?
        if base_starts_with_dot {
            return true;
        }
    }

    // ignore . entries if specified
    if dat.opt.ignore_dot && (base_starts_with_dot || path_contains_slash_dot) {
        return true;
    }

    false
}

pub fn get_real_path(f: &str, dat: &crate::UserData) -> String {
    if dat.opt.abs {
        assert!(util::is_abspath(f));
        f.to_string()
    } else if f == dat.get_input_prefix() {
        return ".".to_string();
    } else if dat.get_input_prefix() == "/" {
        return f[1..].to_string();
    } else if f.starts_with(dat.get_input_prefix()) {
        let f = &f[dat.get_input_prefix().len() + 1..];
        assert!(!f.starts_with('/'));
        return f.to_string();
    } else {
        return f.to_string(); // f is probably symlink target
    }
}

fn print_byte(f: &str, inb: &[u8], dat: &crate::UserData) -> Result<(), std::io::Error> {
    assert_file_path(f, dat);

    // get hash value
    let hash::HashValue { b, .. } = hash::get_byte_hash(inb, &dat.opt.hash_algo)?;
    assert!(!b.is_empty());
    let hex_sum = hash::get_hex_sum(&b);

    // verify hash value if specified
    if !dat.opt.hash_verify.is_empty() && dat.opt.hash_verify != hex_sum {
        return Ok(());
    }

    if dat.opt.hash_only {
        println!("{}", hex_sum);
    } else {
        let realf = get_real_path(f, dat);
        if realf == "." {
            println!("{}", hex_sum);
        } else {
            println!("{}", util::get_xsum_format_string(&realf, &hex_sum));
        }
    }

    Ok(())
}

fn print_file(
    f: &str,
    l: &str,
    t: util::FileType,
    dat: &mut crate::UserData,
) -> Result<(), std::io::Error> {
    assert_file_path(f, dat);
    if !l.is_empty() {
        assert_file_path(f, dat);
    }

    // debug print first
    if dat.opt.debug {
        print_debug(f, t, dat)?;
    }

    // get hash value
    let hash::HashValue { b, written } = hash::get_file_hash(f, &dat.opt.hash_algo)?;
    assert!(!b.is_empty());
    let hex_sum = hash::get_hex_sum(&b);

    // count this file
    dat.stat.append_stat_total();
    dat.stat.append_written_total(written);
    match t {
        util::REG => {
            dat.stat.append_stat_regular(f);
            dat.stat.append_written_regular(written);
        }
        util::DEVICE => {
            dat.stat.append_stat_device(f);
            dat.stat.append_written_device(written);
        }
        _ => util::panic_file_type(f, "invalid", t),
    }

    // verify hash value if specified
    if !dat.opt.hash_verify.is_empty() && dat.opt.hash_verify != hex_sum {
        return Ok(());
    }

    // squash or print this file
    if dat.opt.hash_only {
        if dat.opt.squash {
            dat.squash.update_squash_buffer(&b);
        } else {
            println!("{}", hex_sum);
        }
    } else {
        // make link -> target format if symlink
        let mut realf = get_real_path(f, dat);
        let tmp = String::from(l); // need tmp variable here
        let mut l = tmp.as_str();
        if !l.is_empty() {
            assert_file_path(l, dat);
            if !dat.opt.abs && l.starts_with(dat.get_input_prefix()) {
                l = &l[dat.get_input_prefix().len() + 1..];
                assert!(!l.starts_with('/'));
            }
            realf = format!("{} -> {}", l, realf);
        }
        if dat.opt.squash {
            let mut v = realf.as_bytes().to_vec();
            v.extend(b);
            dat.squash.update_squash_buffer(&v);
        } else {
            println!("{}", util::get_xsum_format_string(&realf, &hex_sum));
        }
    }

    Ok(())
}

fn print_symlink(f: &str, dat: &mut crate::UserData) -> Result<(), std::io::Error> {
    assert_file_path(f, dat);

    // debug print first
    if dat.opt.debug {
        print_debug(f, util::SYMLINK, dat)?;
    }

    // get a symlink string to get hash value
    let s = util::read_link(f)?;

    // get hash value
    let hash::HashValue { b, written } = hash::get_string_hash(&s, &dat.opt.hash_algo)?;
    assert!(!b.is_empty());
    let hex_sum = hash::get_hex_sum(&b);

    // count this file
    dat.stat.append_stat_total();
    dat.stat.append_written_total(written);
    dat.stat.append_stat_symlink(f);
    dat.stat.append_written_symlink(written);

    // verify hash value if specified
    if !dat.opt.hash_verify.is_empty() && dat.opt.hash_verify != hex_sum {
        return Ok(());
    }

    // squash or print this file
    if dat.opt.hash_only {
        if dat.opt.squash {
            dat.squash.update_squash_buffer(&b);
        } else {
            println!("{}", hex_sum);
        }
    } else {
        // hash value is from s, but print realf path for clarity
        let realf = get_real_path(f, dat);
        if dat.opt.squash {
            let mut v = realf.as_bytes().to_vec();
            v.extend(b);
            dat.squash.update_squash_buffer(&v);
        } else {
            println!("{}", util::get_xsum_format_string(&realf, &hex_sum));
        }
    }

    Ok(())
}

fn print_unsupported(f: &str, dat: &mut crate::UserData) -> Result<(), std::io::Error> {
    if dat.opt.debug {
        print_debug(f, util::UNSUPPORTED, dat)?;
    }

    dat.stat.append_stat_unsupported(f);
    Ok(())
}

fn print_invalid(f: &str, dat: &mut crate::UserData) -> Result<(), std::io::Error> {
    if dat.opt.debug {
        print_debug(f, util::INVALID, dat)?;
    }

    dat.stat.append_stat_invalid(f);
    Ok(())
}

fn print_debug(f: &str, t: util::FileType, dat: &crate::UserData) -> Result<(), std::io::Error> {
    assert!(dat.opt.debug);
    let s = util::get_file_type_string(t);
    if dat.opt.abs {
        println!("### {} {}", util::get_abspath(f)?, s);
    } else {
        println!("### {} {}", f, s);
    }

    Ok(())
}

fn print_verbose_stat(dat: &crate::UserData) {
    let indent = String::from(" ");

    util::print_num_format_string(dat.stat.num_stat_total() as usize, "file");
    let a1 = dat.stat.num_stat_regular();
    let a2 = dat.stat.num_stat_device();
    let a3 = dat.stat.num_stat_symlink();
    assert!(a1 + a2 + a3 == dat.stat.num_stat_total());
    if a1 > 0 {
        print!("{}", indent);
        util::print_num_format_string(a1 as usize, util::REG_STR);
    }
    if a2 > 0 {
        print!("{}", indent);
        util::print_num_format_string(a2 as usize, util::DEVICE_STR);
    }
    if a3 > 0 {
        print!("{}", indent);
        util::print_num_format_string(a3 as usize, util::SYMLINK_STR);
    }

    util::print_num_format_string(dat.stat.num_written_total() as usize, "byte");
    let b1 = dat.stat.num_written_regular();
    let b2 = dat.stat.num_written_device();
    let b3 = dat.stat.num_written_symlink();
    assert!(b1 + b2 + b3 == dat.stat.num_written_total());
    if b1 > 0 {
        print!("{}", indent);
        util::print_num_format_string(b1 as usize, &format!("{} {}", util::REG_STR, "byte"));
    }
    if b2 > 0 {
        print!("{}", indent);
        util::print_num_format_string(b2 as usize, &format!("{} {}", util::DEVICE_STR, "byte"));
    }
    if b3 > 0 {
        print!("{}", indent);
        util::print_num_format_string(b3 as usize, &format!("{} {}", util::SYMLINK_STR, "byte"));
    }

    dat.stat.print_stat_ignored(dat);
}

fn assert_file_path(f: &str, dat: &crate::UserData) {
    // must always handle file as abs
    assert!(util::is_abspath(f));

    // file must not end with "/"
    assert!(!f.ends_with('/'));

    // inputPrefix must not end with "/"
    assert!(!dat.get_input_prefix().ends_with('/'));
}
