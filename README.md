# reverse-io

A crate supplying a wrapper type that reverses std::io::{Read,Write,Seek}

```
extern crate reverse_io;
use std::io::{Seek, File, Read, Write};

fn main() -> {
	let f = File::open("hi").unwrap();
	let t = reverse_io::T(f);

	let v = vec![]
	f.read_to_end(&mut v).unwrap();

	println!("{:?}", v);
}
```
