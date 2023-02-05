dirhash-rs ([v0.1.1](https://github.com/kusumi/dirhash-rs/releases/tag/v0.1.1))
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

    $ ./target/debug/dirhash-rs
    usage: ./target/debug/dirhash-rs [<options>] <paths>
    
    Options:
            --hash_algo <string>
                            Hash algorithm to use (default "sha256")
            --hash_verify <string>
                            Message digest to verify in hex string
            --hash_only     Do not print file path
            --ignore_dot    Ignore entry starts with .
            --ignore_dot_dir
                            Ignore directory starts with .
            --ignore_dot_file
                            Ignore file starts with .
            --ignore_symlink
                            Ignore symbolic link
            --lstat         Do not resolve symbolic link
            --abs           Print file path in absolute path
            --squash        Print squashed message digest instead of per file
            --verbose       Enable verbose print
            --debug         Enable debug print
        -v, --version       Print version and exit
        -h, --help          print this help menu

## Bug

This program turned out to be much slower than the original golang implementation. I expected it to be at least as fast as golang.

## Resource

[https://github.com/kusumi/dirhash-rs](https://github.com/kusumi/dirhash-rs)
