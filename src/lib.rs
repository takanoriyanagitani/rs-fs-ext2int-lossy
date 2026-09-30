use std::io;

use std::ffi::OsStr;
use std::path::Path;

use io::BufRead;

use io::BufWriter;
use io::Write;

pub fn path2ext32bits(p: &Path) -> [u8; 4] {
    let oext: Option<&_> = p.extension();
    let obytes: Option<&[u8]> = oext.map(|o| o.as_encoded_bytes());
    let Some(ebytes) = obytes else { return [0; 4] };
    let sz: usize = ebytes.len().min(4);
    let limited: &[u8] = &ebytes[..sz];
    let mut buf: [u8; 4] = [0; 4];
    buf[..sz].copy_from_slice(limited);
    buf
}

#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

#[cfg(target_os = "wasi")]
use std::os::wasi::ffi::OsStrExt;

#[cfg(any(unix, target_os = "wasi"))]
pub fn reader2paths2exts2ints_lossy<R, S>(rdr: R, sep: u8, mut sink: S) -> Result<(), io::Error>
where
    R: BufRead,
    S: FnMut([u8; 4]) -> Result<(), io::Error>,
{
    let splited = rdr.split(sep);
    for rline in splited {
        let line: Vec<u8> = rline?;
        let ostr: &OsStr = OsStr::from_bytes(&line);
        let pat: &Path = Path::new(ostr);
        let lossy: [u8; 4] = path2ext32bits(pat);
        sink(lossy)?;
    }
    Ok(())
}

#[cfg(any(unix, target_os = "wasi"))]
pub fn stdin2paths2exts2ints2stdout_lossy(sep: u8) -> Result<(), io::Error> {
    let o = io::stdout();
    let mut ol = o.lock();
    let mut bw = BufWriter::new(&mut ol);
    reader2paths2exts2ints_lossy(io::stdin().lock(), sep, |lossy: [u8; 4]| {
        bw.write_all(&lossy)?;
        Ok(())
    })?;
    bw.flush()?;
    drop(bw);
    ol.flush()
}

#[cfg(any(unix, target_os = "wasi"))]
pub fn stdin2paths2exts2ints2stdout_default() -> Result<(), io::Error> {
    stdin2paths2exts2ints2stdout_lossy(b'\n')
}

#[cfg(test)]
mod tests {
    mod path2ext32bits {
        use std::path::Path;

        #[test]
        fn noext() {
            let noe = Path::new("/path/to/Dockerfile");
            let extbuf: [u8; 4] = crate::path2ext32bits(noe);
            assert_eq!(extbuf, [0; 4]);
        }

        #[test]
        fn jpg() {
            let jpg = Path::new("/path/to/sample.jpg");
            let extbuf: [u8; 4] = crate::path2ext32bits(jpg);
            assert_eq!(&extbuf, b"jpg\0");
        }

        #[test]
        fn jpeg() {
            let jpg = Path::new("/path/to/sample.jpeg");
            let extbuf: [u8; 4] = crate::path2ext32bits(jpg);
            assert_eq!(&extbuf, b"jpeg");
        }

        #[test]
        fn long_ext_truncated() {
            let long_ext = Path::new("archive.superlong");
            let extbuf: [u8; 4] = crate::path2ext32bits(long_ext);
            assert_eq!(&extbuf, b"supe");
        }

        #[test]
        fn empty_extension() {
            let p = Path::new("file.");
            let extbuf: [u8; 4] = crate::path2ext32bits(p);
            assert_eq!(extbuf, [0; 4]);
        }
    }
}
