mod dir;
mod hash;
mod stat;
mod util;

// squash1
#[cfg(feature = "squash1")]
mod squash1;

#[cfg(feature = "squash1")]
use squash1::{Squash, SQUASH_LABEL, SQUASH_VERSION};

// squash2
#[cfg(feature = "squash2")]
mod squash2;

#[cfg(feature = "squash2")]
use squash2::{Squash, SQUASH_LABEL, SQUASH_VERSION};

const VERSION: [i32; 3] = [0, 4, 5];

#[derive(Debug)]
struct Opt {
    hash_algo: String,
    hash_verify: String,
    hash_only: bool,
    ignore_dot: bool,
    ignore_dot_dir: bool,
    ignore_dot_file: bool,
    ignore_symlink: bool,
    follow_symlink: bool,
    abs: bool,
    swap: bool,
    sort: bool,
    squash: bool,
    verbose: bool,
    debug: bool,
}

impl Default for Opt {
    fn default() -> Self {
        Self {
            hash_algo: "sha256".to_string(),
            hash_verify: String::new(),
            hash_only: false,
            ignore_dot: false,
            ignore_dot_dir: false,
            ignore_dot_file: false,
            ignore_symlink: false,
            follow_symlink: false,
            abs: false,
            swap: false,
            sort: false,
            squash: false,
            verbose: false,
            debug: false,
        }
    }
}

fn get_version_string() -> String {
    format!("{}.{}.{}", VERSION[0], VERSION[1], VERSION[2])
}

fn print_version() {
    println!("{}", get_version_string());
}

fn usage(progname: &str, opts: &getopts::Options) {
    print!(
        "{}",
        opts.usage(&format!("usage: {progname} [<options>] <paths>"))
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let progname = &args[0];

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
    opts.optflag(
        "",
        "follow_symlink",
        "Follow symbolic links unless directory",
    );
    opts.optflag("", "abs", "Print file paths in absolute path");
    opts.optflag("", "swap", "Print file path first in each line");
    opts.optflag("", "sort", "Print sorted file paths");
    opts.optflag(
        "",
        "squash",
        "Print squashed message digest instead of per file",
    );
    opts.optflag("", "verbose", "Enable verbose print");
    opts.optflag("", "debug", "Enable debug print");
    opts.optflag("v", "version", "Print version and exit");
    opts.optflag("h", "help", "Print usage and exit");

    let matches = match opts.parse(&args[1..]) {
        Ok(v) => v,
        Err(e) => {
            println!("{e}");
            usage(progname, &opts);
            std::process::exit(1);
        }
    };
    if matches.opt_present("v") {
        print_version();
        std::process::exit(1);
    }
    if matches.opt_present("h") {
        usage(progname, &opts);
        std::process::exit(1);
    }

    let mut opt = Opt {
        ..Default::default()
    };
    if let Some(v) = matches.opt_str("hash_algo") {
        opt.hash_algo = v;
    }
    if let Some(v) = matches.opt_str("hash_verify") {
        opt.hash_verify = v;
    }
    opt.hash_only = matches.opt_present("hash_only");
    opt.ignore_dot = matches.opt_present("ignore_dot");
    opt.ignore_dot_dir = matches.opt_present("ignore_dot_dir");
    opt.ignore_dot_file = matches.opt_present("ignore_dot_file");
    opt.ignore_symlink = matches.opt_present("ignore_symlink");
    opt.follow_symlink = matches.opt_present("follow_symlink");
    opt.abs = matches.opt_present("abs");
    opt.swap = matches.opt_present("swap");
    opt.sort = matches.opt_present("sort");
    opt.squash = matches.opt_present("squash");
    opt.verbose = matches.opt_present("verbose");
    opt.debug = matches.opt_present("debug");

    if matches.free.is_empty() {
        usage(progname, &opts);
        std::process::exit(1);
    }

    if opt.hash_algo.is_empty() {
        println!("No hash algorithm specified");
        std::process::exit(1);
    }

    if opt.verbose {
        println!("{}", opt.hash_algo);
    }

    if hash::new_hash(&opt.hash_algo).is_err() {
        println!("Unsupported hash algorithm {}", opt.hash_algo);
        println!(
            "Available hash algorithm {:?}",
            hash::get_available_hash_algo()
        );
        std::process::exit(1);
    }

    if !opt.hash_verify.is_empty() {
        let (s, valid) = util::is_valid_hexsum(&opt.hash_verify);
        if !valid {
            println!("Invalid verify string {}", opt.hash_verify);
            std::process::exit(1);
        }
        opt.hash_verify = s.to_string();
    }

    if cfg!(target_os = "windows") {
        assert!(util::is_windows());
        println!("Windows unsupported");
        std::process::exit(1);
    }

    let s = util::get_path_separator();
    if s != '/' {
        println!("Invalid path separator {s}");
        std::process::exit(1);
    }

    let args = matches.free;
    for (i, x) in args.iter().enumerate() {
        if let Err(e) = dir::print_input(x, &opt) {
            panic!("{e}");
        }
        if opt.verbose && !args.is_empty() && i != args.len() - 1 {
            println!();
        }
    }
}
