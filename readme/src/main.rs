extern crate reverse_io;
use std::io::{Seek, Read, Write};
use std::fs::OpenOptions;

fn main() {
    let mut f = OpenOptions::new()
        .read(true).write(true).create(true).open("hi").unwrap();

    f.write_all(b"hello").unwrap();

    let mut t = reverse_io::T(f);

    let mut v = vec![];

    // Creating reverse_io::T doesn't adjust our pos, so we're at the end of the file (and as a
    // result could read immeidately). For kicks, seek to 2 before the "End".
    t.seek(std::io::SeekFrom::End(2)).unwrap();
    t.read_to_end(&mut v).unwrap();

    // Print 'eh'. The first 2 characters in the file, but the last 2 if we view it in reverse.
    println!("{:?} {}", v, String::from_utf8_lossy(&v));
}
