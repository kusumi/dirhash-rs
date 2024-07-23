use crate::hash;
use crate::stat;
use crate::util;
use crate::Opt;
use crate::Squash;
use crate::SQUASH_LABEL;
use crate::SQUASH_VERSION;

pub(crate) fn print_input(f: &str, opt: &Opt) -> std::io::Result<()> {
    // keep symlink input as is
    // XXX but unlike filepath.WalkDir, walkdir::WalkDir resolves symlink
    let f = if util::get_raw_file_type(f)?.is_symlink() {
        f.to_string()
    } else {
        let x = util::canonicalize_path(f)?;
        if x.is_empty() {
            return Ok(());
        }
        // assert exists
        util::path_exists_or_error(&x)?;
        x
    };

    // convert input to abs first
    let f = util::get_abspath(&f)?;
    assert_file_path(&f, "");

    // keep input prefix based on raw type
    let inp = match util::get_raw_file_type(&f)? {
        util::FileType::Dir => f.clone(),
        util::FileType::Reg | util::FileType::Device | util::FileType::Symlink => {
            util::get_dirpath(&f)?
        }
        _ => return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput)),
    };

    // prefix is a directory
    assert!(util::get_file_type(&inp)?.is_dir());

    // start directory walk
    let mut squ = Squash::new();
    let mut sta = stat::Stat::new();
    walk_directory(&f, &inp, &mut squ, &mut sta, opt)?;

    // print various stats
    if opt.verbose {
        print_verbose_stat(&inp, &mut sta, opt)?;
    }
    sta.print_stat_unsupported(&inp, opt)?;
    sta.print_stat_invalid(&inp, opt)?;

    // print squash hash if specified
    if opt.squash {
        let b = squ.get_buffer();
        if opt.verbose {
            util::print_num_format_string(b.len(), "squashed byte");
        }
        print_byte(&f, &b, &inp, opt)?;
    }
    Ok(())
}

// walkdir::WalkDir has different traversal order vs filepath.WalkDir,
// hence squash2 hash won't match the original golang implementation.
fn walk_directory(
    f: &str,
    inp: &str,
    squ: &mut Squash,
    sta: &mut stat::Stat,
    opt: &Opt,
) -> std::io::Result<()> {
    let mut l = vec![];
    for entry in walkdir::WalkDir::new(f)
        .into_iter()
        .filter_map(std::result::Result::ok)
    {
        let Some(f) = entry.path().to_str() else {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
        };
        if opt.sort {
            l.push(f.to_string());
        } else {
            walk_directory_impl(f, inp, squ, sta, opt)?;
        }
    }
    if opt.sort {
        l.sort();
        for f in &l {
            walk_directory_impl(f, inp, squ, sta, opt)?;
        }
    }
    Ok(())
}

fn walk_directory_impl(
    f: &str,
    inp: &str,
    squ: &mut Squash,
    sta: &mut stat::Stat,
    opt: &Opt,
) -> std::io::Result<()> {
    let mut t = util::get_raw_file_type(f)?;
    if test_ignore_entry(f, &t, opt) {
        sta.append_stat_ignored(f);
        return Ok(());
    }

    // find target if symlink
    // l is symlink itself, not its target
    let (x, l) = if t.is_symlink() {
        if opt.ignore_symlink {
            sta.append_stat_ignored(f);
            return Ok(());
        }
        if !opt.follow_symlink {
            print_symlink(f, inp, squ, sta, opt)?;
            return Ok(());
        }
        let x = util::canonicalize_path(f)?;
        if x.is_empty() {
            print_invalid(f, sta, opt)?;
            return Ok(());
        }
        assert!(util::is_abspath(&x));
        t = util::get_file_type(&x)?; // update type
        assert!(!t.is_symlink()); // symlink chains resolved
        (x, f.to_string())
    } else {
        (f.to_string(), String::new())
    };

    match t {
        util::FileType::Dir => handle_directory(&x, &l, inp, squ, sta, opt)?,
        util::FileType::Reg | util::FileType::Device => print_file(&x, &l, &t, inp, squ, sta, opt)?,
        util::FileType::Unsupported => print_unsupported(&x, sta, opt)?,
        util::FileType::Invalid => print_invalid(&x, sta, opt)?,
        util::FileType::Symlink => util::panic_file_type(&x, "symlink", &t),
    }
    Ok(())
}

fn test_ignore_entry(f: &str, t: &util::FileType, opt: &Opt) -> bool {
    assert!(util::is_abspath(f));

    // only non directory types count
    if t.is_dir() {
        return false;
    }

    let base_starts_with_dot = match util::get_basename(f) {
        Ok(v) => v.starts_with('.'),
        Err(_) => false,
    };
    let path_contains_slash_dot = f.contains("/.");

    // ignore . directories if specified
    if opt.ignore_dot_dir && !base_starts_with_dot && path_contains_slash_dot {
        return true;
    }

    // ignore . regular files if specified
    if opt.ignore_dot_file {
        // XXX limit to REG ?
        if base_starts_with_dot {
            return true;
        }
    }

    // ignore . entries if specified
    opt.ignore_dot && (base_starts_with_dot || path_contains_slash_dot)
}

fn trim_input_prefix<'a>(f: &'a str, inp: &'a str) -> &'a str {
    if f.starts_with(inp) {
        let f = &f[inp.len() + 1..];
        assert!(!f.starts_with('/'));
        f
    } else {
        f
    }
}

pub(crate) fn get_real_path<'a>(f: &'a str, inp: &'a str, opt: &'a Opt) -> &'a str {
    if opt.abs {
        assert!(util::is_abspath(f));
        f
    } else if f == inp {
        "."
    } else if inp == "/" {
        &f[1..]
    } else {
        // f is probably symlink target if f unchanged
        trim_input_prefix(f, inp)
    }
}

fn print_byte(f: &str, inb: &[u8], inp: &str, opt: &Opt) -> std::io::Result<()> {
    assert_file_path(f, inp);

    // get hash value
    let (b, ..) = hash::get_byte_hash(inb, &opt.hash_algo)?;
    assert!(!b.is_empty());
    let hex_sum = hash::get_hex_sum(&b);

    // verify hash value if specified
    if !opt.hash_verify.is_empty() && opt.hash_verify != hex_sum {
        return Ok(());
    }

    if opt.hash_only {
        println!("{hex_sum}");
    } else {
        // no space between two
        let s = format!("[{SQUASH_LABEL}][v{SQUASH_VERSION}]");
        let realf = get_real_path(f, inp, opt);
        if realf == "." {
            println!("{hex_sum}{s}");
        } else {
            println!(
                "{}{}",
                util::get_xsum_format_string(realf, &hex_sum, opt.swap),
                s
            );
        }
    }
    Ok(())
}

fn handle_directory<'a>(
    f: &str,
    l: &'a str,
    inp: &'a str,
    squ: &mut Squash,
    sta: &mut stat::Stat,
    opt: &Opt,
) -> std::io::Result<()> {
    assert_file_path(f, inp);
    if !l.is_empty() {
        assert_file_path(l, inp);
    }

    // nothing to do if input is input prefix
    if f == inp {
        return Ok(());
    }

    // nothing to do unless squash
    if !opt.squash {
        return Ok(());
    }

    // debug print first
    if opt.debug {
        print_debug(f, &util::FileType::Dir, opt)?;
    }

    // get hash value
    // path must be relative to input prefix
    let s = trim_input_prefix(f, inp);
    let (b, written) = hash::get_string_hash(s, &opt.hash_algo)?;
    assert!(!b.is_empty());

    // count this file
    sta.append_stat_total();
    sta.append_written_total(written);
    sta.append_stat_directory(f);
    sta.append_written_directory(written);

    // squash
    assert!(opt.squash);
    if opt.hash_only {
        squ.update_buffer(&b)?;
    } else {
        // make link -> target format if symlink
        let mut realf = get_real_path(f, inp, opt).to_string();
        if !l.is_empty() {
            assert_file_path(l, inp);
            let mut l = l;
            if !opt.abs {
                l = trim_input_prefix(l, inp);
                assert!(!l.starts_with('/'));
            }
            realf = format!("{l} -> {realf}");
        }
        let mut v = realf.as_bytes().to_vec();
        v.extend(b);
        squ.update_buffer(&v)?;
    }
    Ok(())
}

fn print_file<'a>(
    f: &str,
    l: &'a str,
    t: &util::FileType,
    inp: &'a str,
    squ: &mut Squash,
    sta: &mut stat::Stat,
    opt: &Opt,
) -> std::io::Result<()> {
    assert_file_path(f, inp);
    if !l.is_empty() {
        assert_file_path(l, inp);
    }

    // debug print first
    if opt.debug {
        print_debug(f, t, opt)?;
    }

    // get hash value
    let (b, written) = hash::get_file_hash(f, &opt.hash_algo)?;
    assert!(!b.is_empty());
    let hex_sum = hash::get_hex_sum(&b);

    // count this file
    sta.append_stat_total();
    sta.append_written_total(written);
    match t {
        util::FileType::Reg => {
            sta.append_stat_regular(f);
            sta.append_written_regular(written);
        }
        util::FileType::Device => {
            sta.append_stat_device(f);
            sta.append_written_device(written);
        }
        _ => util::panic_file_type(f, "invalid", t),
    }

    // verify hash value if specified
    if !opt.hash_verify.is_empty() && opt.hash_verify != hex_sum {
        return Ok(());
    }

    // squash or print this file
    if opt.hash_only {
        if opt.squash {
            squ.update_buffer(&b)?;
        } else {
            println!("{hex_sum}");
        }
    } else {
        // make link -> target format if symlink
        let mut realf = get_real_path(f, inp, opt).to_string();
        if !l.is_empty() {
            assert_file_path(l, inp);
            let mut l = l;
            if !opt.abs {
                l = trim_input_prefix(l, inp);
                assert!(!l.starts_with('/'));
            }
            realf = format!("{l} -> {realf}");
        }
        if opt.squash {
            let mut v = realf.as_bytes().to_vec();
            v.extend(b);
            squ.update_buffer(&v)?;
        } else {
            println!(
                "{}",
                util::get_xsum_format_string(&realf, &hex_sum, opt.swap)
            );
        }
    }
    Ok(())
}

fn print_symlink(
    f: &str,
    inp: &str,
    squ: &mut Squash,
    sta: &mut stat::Stat,
    opt: &Opt,
) -> std::io::Result<()> {
    assert_file_path(f, inp);

    // debug print first
    if opt.debug {
        print_debug(f, &util::FileType::Symlink, opt)?;
    }

    // get hash value of symlink base name
    let (b, written) = hash::get_string_hash(&util::get_basename(f)?, &opt.hash_algo)?;
    assert!(!b.is_empty());
    let hex_sum = hash::get_hex_sum(&b);

    // count this file
    sta.append_stat_total();
    sta.append_written_total(written);
    sta.append_stat_symlink(f);
    sta.append_written_symlink(written);

    // verify hash value if specified
    if !opt.hash_verify.is_empty() && opt.hash_verify != hex_sum {
        return Ok(());
    }

    // squash or print this file
    if opt.hash_only {
        if opt.squash {
            squ.update_buffer(&b)?;
        } else {
            println!("{hex_sum}");
        }
    } else {
        let realf = get_real_path(f, inp, opt);
        if opt.squash {
            let mut v = realf.as_bytes().to_vec();
            v.extend(b);
            squ.update_buffer(&v)?;
        } else {
            println!(
                "{}",
                util::get_xsum_format_string(realf, &hex_sum, opt.swap)
            );
        }
    }
    Ok(())
}

fn print_unsupported(f: &str, sta: &mut stat::Stat, opt: &Opt) -> std::io::Result<()> {
    if opt.debug {
        print_debug(f, &util::FileType::Unsupported, opt)?;
    }
    sta.append_stat_unsupported(f);
    Ok(())
}

fn print_invalid(f: &str, sta: &mut stat::Stat, opt: &Opt) -> std::io::Result<()> {
    if opt.debug {
        print_debug(f, &util::FileType::Invalid, opt)?;
    }
    sta.append_stat_invalid(f);
    Ok(())
}

fn print_debug(f: &str, t: &util::FileType, opt: &Opt) -> std::io::Result<()> {
    assert!(opt.debug);
    if opt.abs {
        println!("### {} {}", util::get_abspath(f)?, t.as_str());
    } else {
        println!("### {} {}", f, t.as_str());
    }
    Ok(())
}

fn print_verbose_stat(inp: &str, sta: &mut stat::Stat, opt: &Opt) -> std::io::Result<()> {
    let indent = " ";

    util::print_num_format_string(sta.num_stat_total(), "file");
    let a0 = sta.num_stat_directory();
    let a1 = sta.num_stat_regular();
    let a2 = sta.num_stat_device();
    let a3 = sta.num_stat_symlink();
    assert_eq!(a0 + a1 + a2 + a3, sta.num_stat_total());
    if a0 > 0 {
        print!("{indent}");
        util::print_num_format_string(a0, util::FileType::Dir.as_str());
    }
    if a1 > 0 {
        print!("{indent}");
        util::print_num_format_string(a1, util::FileType::Reg.as_str());
    }
    if a2 > 0 {
        print!("{indent}");
        util::print_num_format_string(a2, util::FileType::Device.as_str());
    }
    if a3 > 0 {
        print!("{indent}");
        util::print_num_format_string(a3, util::FileType::Symlink.as_str());
    }

    util::print_num_format_string(sta.num_written_total(), "byte");
    let b0 = sta.num_written_directory();
    let b1 = sta.num_written_regular();
    let b2 = sta.num_written_device();
    let b3 = sta.num_written_symlink();
    assert_eq!(b0 + b1 + b2 + b3, sta.num_written_total());
    if b0 > 0 {
        print!("{indent}");
        util::print_num_format_string(b0, &format!("{} {}", util::FileType::Dir.as_str(), "byte"));
    }
    if b1 > 0 {
        print!("{indent}");
        util::print_num_format_string(b1, &format!("{} {}", util::FileType::Reg.as_str(), "byte"));
    }
    if b2 > 0 {
        print!("{indent}");
        util::print_num_format_string(
            b2,
            &format!("{} {}", util::FileType::Device.as_str(), "byte"),
        );
    }
    if b3 > 0 {
        print!("{indent}");
        util::print_num_format_string(
            b3,
            &format!("{} {}", util::FileType::Symlink.as_str(), "byte"),
        );
    }

    sta.print_stat_ignored(inp, opt)
}

fn assert_file_path(f: &str, inp: &str) {
    // must always handle file as abs
    assert!(util::is_abspath(f));

    // file must not end with "/"
    assert!(!f.ends_with('/'));

    // inputPrefix must not end with "/"
    assert!(!inp.ends_with('/'));
}
