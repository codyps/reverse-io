#![feature(io)]

use std::{cmp, io};
use std::io::SeekFrom;
use std::io::{Seek,Read,Write};

pub struct T<S>(pub S);

impl<S: Read + Seek> Read for T<S> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let pos = try!(self.0.seek(SeekFrom::Current(0)));
        let t_dist = -(cmp::min(buf.len(), pos as usize) as i64);
        let n_pos = try!(self.0.seek(SeekFrom::Current(t_dist)));
        let dist = (pos - n_pos) as usize;
        let ct = try!(self.0.read(&mut buf[..dist]));
        /* XXX: we have the data at this point, even if this seek fails. Should we ignore a Seek
         * failure? */
        try!(self.0.seek(SeekFrom::Start(n_pos)));
        buf[..ct].reverse();
        Ok(ct)
    }
}

impl<S: Write + io::Seek> Write for T<S> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let pos = try!(self.0.seek(SeekFrom::Current(0)));
        let seek_goal = cmp::min(buf.len(), pos as usize) as i64;
        let new_pos = try!(self.0.seek(SeekFrom::Current(-seek_goal)));
        let seek_dist = (pos - new_pos) as usize;
        /* XXX: it may make sense for dist != buf.len() being an error, and not performing a
         * partial write */
        let mut r_buf = buf[..seek_dist].to_vec();
        r_buf.reverse();
        /* Note that we must use write_all(), and even then partial writes are problematic as they
         * go the wrong way.
         */
        try!(self.0.write_all(&r_buf));
        try!(self.0.seek(SeekFrom::Start(new_pos)));
        Ok(seek_dist)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

impl<S: io::Seek> io::Seek for T<S> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let real_pos = match pos {
            /* TODO: consider whether we should allow seeking before the start */
            SeekFrom::Current(x) => SeekFrom::Current(-x),
            SeekFrom::Start(x) => {
                assert!(x <= std::i64::MAX as u64);
                SeekFrom::End(-(x as i64))
            },
            SeekFrom::End(x) => {
                assert!(x >= 0);
                SeekFrom::Start(x as u64)
            },
        };

        let u_res = try!(self.0.seek(real_pos));

        let cur = try!(self.0.seek(SeekFrom::Current(0)));
        let end = try!(self.0.seek(SeekFrom::End(0)));
        try!(self.0.seek(SeekFrom::Start(cur)));
        Ok(end - u_res)
    }
}

#[test]
fn seek() {
    use std::io::Cursor;
    let mut c = T(Cursor::new(vec![4u8, 6, 2]));
    assert_eq!(c.seek(SeekFrom::Start(0)).unwrap(), 0);
    assert_eq!(c.seek(SeekFrom::Start(1)).unwrap(), 1);
    assert_eq!(c.seek(SeekFrom::End(0)).unwrap(), 3);
    assert_eq!(c.seek(SeekFrom::Current(-1)).unwrap(), 2);
}

#[test]
fn read() {
    use std::io::Cursor;
    let t = vec![4u8, 6, 2];
    let mut r = t.clone();
    r.reverse();

    let mut c = T(Cursor::new(t));
    let mut b = [0; 3];
    assert_eq!(c.seek(SeekFrom::Start(0)).unwrap(), 0);
    assert_eq!(c.read(&mut b).unwrap(), 3);
    assert_eq!(b, r);
    assert_eq!(c.seek(SeekFrom::Current(0)).unwrap(), 3);
    assert_eq!(c.read(&mut b).unwrap(), 0);
    assert_eq!(b, r);
}

#[test]
fn write() {
    use std::io::Cursor;
    let t = vec![4u8, 6, 2];

    let mut c = T(Cursor::new(t));
    let b = [5, 2, 6];
    let mut r = b;
    r.reverse();
    assert_eq!(c.seek(SeekFrom::Start(0)).unwrap(), 0);
    assert_eq!(c.write(&b).unwrap(), 3);
    assert_eq!(&c.0.get_ref()[..], &r);
    assert_eq!(c.write(&b).unwrap(), 0);
    assert_eq!(c.seek(SeekFrom::Start(0)).unwrap(), 0);
    assert_eq!(c.write(&b[..2]).unwrap(), 2);
    assert_eq!(c.write(&b[..2]).unwrap(), 1);
}
