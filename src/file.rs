use std::convert::AsRef;
use std::iter::IntoIterator;
use std::path::Path;
use std::io::{Result as IoResult, Error as IoError};
use std::io::{Write, Read, BufRead, Lines, BufReader};
use std::fs::{OpenOptions, File, remove_file};
use std::fs::copy as fs_copy;

#[cfg(windows)]
const LINE_SEP: &'static [u8] = b"\r\n";

#[cfg(not(windows))]
const LINE_SEP: &'static [u8] = b"\n";

/// Writes `content` into a file at `path`.
///
/// If the file at `path` does not exist, it will be created.
/// Otherwise, the file will be completely overwritten.
///
/// ```rust,no_run
/// // write a string
/// latin::file::write("./foo.txt", "contents");
/// // write bytes
/// latin::file::write("./foo.txt", &[5u8, 10u8]);
/// ```
pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> IoResult<()> {
    let mut file = try!(OpenOptions::new().write(true).create(true).truncate(true).open(path));
    file.write_all(contents.as_ref())
}

/// Writes lines into a file at `path`.
///
/// If the file at `path` does not exist, it will be created.
/// Otherwise, the file will be completely overwritten.
///
/// ```rust,no_run
/// latin::file::write_lines("./foo.txt", vec!["line1", "line2"]);
/// // works with iterators too
/// latin::file::write_lines("./foo.txt", vec!["line1", "line2"].iter().map(|l| &l[0 .. 2]));
/// ```
pub fn write_lines<P: AsRef<Path>, I: IntoIterator<Item=B, IntoIter=A>, A: Iterator<Item=B>, B: AsRef<[u8]>>(path: P, lines: I) -> IoResult<()> {
    let mut file = try!(OpenOptions::new().write(true).create(true).truncate(true).open(path));
    for line in lines.into_iter() {
        try!(file.write_all(line.as_ref()));
        try!(file.write_all(LINE_SEP));
    }
    Ok(())
}

/// Appends some contents to the file at `path`.
///
/// If the file at `path` does not exist, it will be created.
/// Otherwise, the file will be completely overwritten.
///
/// ```rust,no_run
/// // append a string
/// latin::file::append("./foo.txt", "appended content");
/// // append bytes
/// latin::file::append("./foo.txt", &[10u8, 5u8]);
/// ```
pub fn append<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> IoResult<()> {
    let mut file = try!(OpenOptions::new().write(true).create(true).append(true).open(path));
    file.write_all(contents.as_ref())
}

/// Appends some contents followed by a newline to the file at `path`.
///
/// The newline seperator will vary depending on operating system.
/// Windows: "\r\n" All other: "\n"
///
/// If the file at `path` does not exist, it will be created.
/// Otherwise, the file will be completely overwritten.
///
/// ```rust,no_run
/// // append a line followed by a newline
/// latin::file::append_line("./foo.txt", "appended line");
/// // not sure why you'd want this, but this works too
/// latin::file::append_line("./foo.txt", &[10u8, 5u8]);
/// ```
pub fn append_line<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> IoResult<()> {
    let mut file = try!(OpenOptions::new().write(true).create(true).append(true).open(path));
    try!(file.write_all(contents.as_ref()));
    file.write_all(LINE_SEP)
}

/// Reads the file at `path` and returns the content as a Vec<u8>
///
/// If the file at `path` does not exist, an error is returned.
///
/// ```rust,no_run
///# #![allow(unused)]
/// let contents = latin::file::read("foo.txt");
/// // or as a string
/// let contents = latin::file::read("foo.txt").map(String::from_utf8);
/// ```
pub fn read<P: AsRef<Path>>(path: P) -> IoResult<Vec<u8>> {
    let mut file = try!(OpenOptions::new().read(true).open(path));
    let mut out = vec![];
    try!(file.read_to_end(&mut out));
    Ok(out)
}

/// Reads the file at `path` and returns the content as a String
///
/// If the file at `path` does not exist or the file is not utf8,
/// an error is returned.
///
/// ```rust,no_run
///# #![allow(unused)]
/// let contents = latin::file::read_string_utf8("foo.txt");
/// ```
pub fn read_string_utf8<P: AsRef<Path>>(path: P) -> IoResult<String> {
    match read(path)  {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(s) => Ok(s),
            Err(e) => Err(IoError::new(::std::io::ErrorKind::Other, e)),
        },
        Err(e) => Err(e)
    }
}

/// Reads the file at `path` and returns the content as a String
///
/// If the file at `path` does not exist an error is returned.
///
/// Any non-utf8 characters are stripped from the result.  Please
/// see `std::String::from_utf8_lossy` for more info.
///
/// ```rust,no_run
///# #![allow(unused)]
/// let contents = latin::file::read_string_utf8_lossy("foo.txt");
/// ```
pub fn read_string_utf8_lossy<P: AsRef<Path>>(path: P) -> IoResult<String> {
    match read(path)  {
        Ok(bytes) => Ok(String::from_utf8_lossy(&bytes[..]).into_owned()),
        Err(e) => Err(e)
    }
}

/// Reads all of the lines in a file at `path`.
///
/// Returns an iterator over the lines.
///
///
/// ```rust,no_run
/// for line in latin::file::read_lines("foo.txt").unwrap() {
///     println!("{}", line.unwrap());
/// }
/// ```
pub fn read_lines<P: AsRef<Path>>(path: P) -> IoResult<Lines<BufReader<File>>> {
    let file = try!(OpenOptions::new().read(true).open(path));
    let file = BufReader::new(file);
    Ok(file.lines())
}

/// Returns true if `path` exists and is a file.
///
/// ```rust,no_run
/// if latin::file::exists("foo.txt") {
///     // do stuff
/// }
pub fn exists<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    path.is_file() && path.exists()
}

/// Copies a file from `from` to `to`.
///
/// If the file at `to` does not exist, it will be created.
/// Otherwise, the file will be completely overwritten.
///
/// ```rust,no_run
/// latin::file::copy("foo.txt", "bar.txt");
/// ```
pub fn copy<Fp: AsRef<Path>, Tp: AsRef<Path>>(from: Fp, to: Tp) -> IoResult<()> {
    fs_copy(from, to).map(|_| ())
}

/// Removes the file at `path`.
///
/// An error is removed if `path` is not a file, or if
/// the file could not be removed for filesystem reasons.
///
/// ```rust,no_run
/// latin::file::remove("./foo.txt");
/// ```
pub fn remove<P: AsRef<Path>>(path: P) -> IoResult<()> {
    remove_file(path)
}

/// Checks to see if the file at `path` has the file extension `ext`.
pub fn has_extension<P: AsRef<Path>, S: AsRef<str>>(path: P, ext: S) -> bool {
    path.as_ref().extension()
                 .and_then(|ext| ext.to_str())
                 .map(|provided_ext| provided_ext == ext.as_ref())
                 .unwrap_or(false)
}
