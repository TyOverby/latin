use std::convert::AsRef;
use std::iter::IntoIterator;
use std::path::Path;
use std::io::Result as IoResult;
use std::io::copy as io_copy;
use std::io::{Write, Read, BufRead, Lines, BufReader};
use std::fs::{OpenOptions, File, remove_file};

#[cfg(windows)]
#[inline(always)]
const LINE_SEP: &'static [u8] = b"\r\n";

#[cfg(not(windows))]
#[inline(always)]
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
    let mut file = try!(OpenOptions::new().write(true).open(path));
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
    let mut file = try!(OpenOptions::new().write(true).append(true).open(path));
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
    let mut file = try!(OpenOptions::new().write(true).append(true).open(path));
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
    let mut file = try!(OpenOptions::new().write(true).append(true).open(path));
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

/// Returns true if the file at `path` exists.
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
    let mut from = try!(OpenOptions::new().read(true).open(from));
    let mut to = try!(OpenOptions::new().write(true).truncate(true).open(to));
    io_copy(&mut from, &mut to).map(|_| ())
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
