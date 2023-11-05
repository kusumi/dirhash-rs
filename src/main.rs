mod dir;
mod hash;
mod stat;
mod util;

// squash1
#[cfg(feature = "squash1")]
mod squash1;

#[cfg(feature = "squash1")]
use squash1::*;

// squash2
#[cfg(feature = "squash2")]
mod squash2;

#[cfg(feature = "squash2")]
use squash2::*;

const VERSION: [i32; 3] = [0, 4, 0];

#[derive(Debug)]
struct UserOption {
    hash_algo: String,
    hash_verify: String,
    hash_only: bool,
    ignore_dot: bool,
    ignore_dot_dir: bool,
    ignore_dot_file: bool,
    ignore_symlink: bool,
    lstat: bool,
    abs: bool,
    squash: bool,
    verbose: bool,
    debug: bool,
}

impl Default for UserOption {
    fn default() -> UserOption {
        UserOption {
            hash_algo: "sha256".to_string(),
            hash_verify: "".to_string(),
            hash_only: false,
            ignore_dot: false,
            ignore_dot_dir: false,
            ignore_dot_file: false,
            ignore_symlink: false,
            lstat: false,
            abs: false,
            squash: false,
            verbose: false,
            debug: false,
        }
    }
}

#[derive(Debug)]
pub struct UserData {
    opt: UserOption,
    stat: stat::Stat,
    squash: Squash,
    input_prefix: String,
}

impl Default for UserData {
    fn default() -> UserData {
        UserData {
            opt: UserOption::default(),
            stat: stat::Stat::default(),
            squash: Squash::default(),
            input_prefix: "".to_string(),
        }
    }
}

fn get_version_string() -> String {
    format!("{}.{}.{}", VERSION[0], VERSION[1], VERSION[2])
}

fn print_version() {
    println!("{}", get_version_string());
}

fn usage(progname: &str, opts: getopts::Options) {
    print!(
        "{}",
        opts.usage(&format!("usage: {} [<options>] <paths>", progname))
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let progname = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optopt(
        "",
        "hash_algo",
        "Hash algorithm to use (default \"sha256\")",
        "<string>",
    );
    opts.optopt(
        "",
        "hash_verify",
        "Message digest to verify in hex string",
        "<string>",
    );
    opts.optflag("", "hash_only", "Do not print file paths");
    opts.optflag("", "ignore_dot", "Ignore entries start with .");
    opts.optflag("", "ignore_dot_dir", "Ignore directories start with .");
    opts.optflag("", "ignore_dot_file", "Ignore files start with .");
    opts.optflag("", "ignore_symlink", "Ignore symbolic links");
    opts.optflag("", "lstat", "Do not resolve symbolic links");
    opts.optflag("", "abs", "Print file paths in absolute path");
    opts.optflag(
        "",
        "squash",
        "Print squashed message digest instead of per file",
    );
    opts.optflag("", "verbose", "Enable verbose print");
    opts.optflag("", "debug", "Enable debug print");
    opts.optflag("v", "version", "Print version and exit");
    opts.optflag("h", "help", "Print usage and exit");

    let matches = opts.parse(&args[1..]).unwrap();
    if matches.opt_present("v") {
        print_version();
        std::process::exit(1);
    }
    if matches.opt_present("h") {
        usage(&progname, opts);
        std::process::exit(1);
    }

    let mut dat = UserData {
        ..Default::default()
    };
    if matches.opt_present("hash_algo") {
        dat.opt.hash_algo = matches.opt_str("hash_algo").unwrap();
    }
    if matches.opt_present("hash_verify") {
        dat.opt.hash_verify = matches.opt_str("hash_verify").unwrap();
    }
    dat.opt.hash_only = matches.opt_present("hash_only");
    dat.opt.ignore_dot = matches.opt_present("ignore_dot");
    dat.opt.ignore_dot_dir = matches.opt_present("ignore_dot_dir");
    dat.opt.ignore_dot_file = matches.opt_present("ignore_dot_file");
    dat.opt.ignore_symlink = matches.opt_present("ignore_symlink");
    dat.opt.lstat = matches.opt_present("lstat");
    dat.opt.abs = matches.opt_present("abs");
    dat.opt.squash = matches.opt_present("squash");
    dat.opt.verbose = matches.opt_present("verbose");
    dat.opt.debug = matches.opt_present("debug");

    if dat.opt.hash_algo.is_empty() {
        println!("No hash algorithm specified");
        std::process::exit(1);
    }

    if dat.opt.verbose {
        print_version();
        println!("{}", dat.opt.hash_algo);
    }

    if let hash::HashObj::None = hash::new_hash(&dat.opt.hash_algo) {
        println!("Unsupported hash algorithm {}", dat.opt.hash_algo);
        println!(
            "Available hash algorithm {:?}",
            hash::get_available_hash_algo()
        );
        std::process::exit(1);
    }

    if !dat.opt.hash_verify.is_empty() {
        let (s, valid) = util::is_valid_hexsum(&dat.opt.hash_verify);
        if !valid {
            println!("Invalid verify string {}", dat.opt.hash_verify);
            std::process::exit(1);
        }
        dat.opt.hash_verify = s.to_string();
    }

    // incompatible debug prints vs dirhash
    /*
    if dat.opt.debug {
        println!("{}: {:?}", stringify!(main), dat);
        println!(
            "{}: {:?}",
            stringify!(main),
            hash::get_available_hash_algo()
        );
    }
    */

    if cfg!(target_os = "windows") {
        assert!(util::is_windows());
        println!("Windows unsupported");
        std::process::exit(1);
    }

    let s = util::get_path_separator();
    if s != '/' {
        println!("Invalid path separator {}", s);
        std::process::exit(1);
    }

    if matches.free.is_empty() {
        usage(&progname, opts);
        std::process::exit(1);
    }

    let args = matches.free;
    for (i, x) in args.iter().enumerate() {
        let f = util::canonicalize_path(x).unwrap();
        dir::print_input(&f, &mut dat).unwrap();
        if dat.opt.verbose && !args.is_empty() && i != args.len() - 1 {
            println!();
        }
    }
}
