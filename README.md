# reverse-io

[Documentation](http://codyps.com/docs/reverse-io/x86_64-unknown-linux-gnu/stable/reverse_io/struct.T.html)
[![Build Status](https://travis-ci.org/jmesmon/reverse-io.svg?branch=master)](https://travis-ci.org/jmesmon/reverse-io)
[![Crates.io](https://img.shields.io/crates/v/reverse-io.svg?maxAge=2592000)](https://crates.io/crates/reverse-io)


A crate supplying a wrapper type that reverses std::io::{Read,Write,Seek}

```
extern crate reverse_io;
use std::io::{Seek, File, Read, Write};

fn main() {
	let f = File::open("hi").unwrap();
	let t = reverse_io::T(f);

	let v = vec![];
	t.seek(std::io::SeekFrom::Start(0)).unwrap();
	t.read_to_end(&mut v).unwrap();

	println!("{:?}", v);
}
```
