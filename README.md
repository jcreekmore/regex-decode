regex-decode
===

A Rust library for extracting regex captures into a struct.

[![Build Status](https://travis-ci.org/jcreekmore/regex-decode.svg?branch=master)](https://travis-ci.org/jcreekmore/regex-decode)

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
regex-decode = "0.1"
```

and this to your crate root:

```rust
extern crate regex_decode;
```

Here is a simple example that extracts named captures into a struct.

```rust
extern crate regex;
extern crate regex_decode;
extern crate rustc_serialize;

use regex::Regex;
use regex_decode::decode;

#[derive(RustcDecodable)]
struct Capture {
    pub title: String,
    pub year: usize,
}

fn test() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)")
                       .unwrap();
    let text = "Not my favorite movie: 'Citizen Kane' (1941).";

    let val = decode::<Capture>(&re, &text).unwrap();

    assert_eq!(&val.title, "Citizen Kane");
    assert_eq!(val.year, 1941);
}

```

You can also extract to a tuple of you don't want to create a named struct.

```rust
extern crate regex;
extern crate regex_decode;
extern crate rustc_serialize;

use regex::Regex;
use regex_decode::decode;

fn test() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)")
                       .unwrap();
    let text = "Not my favorite movie: 'Citizen Kane' (1941).";

    let (title, year) = decode::<(String, usize)>(&re, &text).unwrap();

    assert_eq!(&title, "Citizen Kane");
    assert_eq!(year, 1941);
}

```
