dirhash-rs ([v0.1.2](https://github.com/kusumi/dirhash-rs/releases/tag/v0.1.2))
========

## About

+ Recursively walk directory trees and print message digest of regular files.

+ Rust version of [https://github.com/kusumi/dirhash](https://github.com/kusumi/dirhash).

## Supported platforms

Unix-likes in general

## Requirements

Recent version of Rust

## Build

    $ make

or

    $ gmake

## Usage

    $ ./target/release/dirhash-rs
    usage: ./target/release/dirhash-rs [<options>] <paths>
    
    Options:
            --hash_algo <string>
                            Hash algorithm to use (default "sha256")
            --hash_verify <string>
                            Message digest to verify in hex string
            --hash_only     Do not print file paths
            --ignore_dot    Ignore entries start with .
            --ignore_dot_dir
                            Ignore directories start with .
            --ignore_dot_file
                            Ignore files start with .
            --ignore_symlink
                            Ignore symbolic links
            --lstat         Do not resolve symbolic links
            --abs           Print file paths in absolute path
            --squash        Print squashed message digest instead of per file
            --verbose       Enable verbose print
            --debug         Enable debug print
        -v, --version       Print version and exit
        -h, --help          Print usage and exit

## Resource

[https://github.com/kusumi/dirhash-rs](https://github.com/kusumi/dirhash-rs)
