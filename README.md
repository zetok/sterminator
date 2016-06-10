# Sterminator
[![Build Status](https://travis-ci.org/zetok/sterminator.svg?branch=master)](https://travis-ci.org/zetok/sterminator)

**Sterminator** replaces text enclosed in triple brackets (e.g.
`[[[text]]]`) with string provided in corresponding JSON `key`.

In case where JSON string is empty, `key` is used for replacement. 

## CLI usage

E.g.
```bash
./sterminator web_file json_file
```

## Packages / builds

### Generic

* [Linux x86_64](https://github.com/zetok/sterminator/releases)


## Building
Fairly simple. You'll need [Rust].

When you'll have deps, build debug version with
```bash
cargo build
```

`sterminator` executable will be located in `target/debug/`

## Support

If you like Sterminator, feel free to help it by contributing, whether that would be
by writing code, suggesting improvements, or by donating.

Donate via Bitcoin: `1FSDbXVbUZSe34UqxJjfNMdAA9P8c6tNFQ`

*If you're interested in some other way of donating, please say so. :smile:*

## License

Licensed under GPLv3+. For details, see [COPYING](/COPYING).

[Rust]: https://www.rust-lang.org/
