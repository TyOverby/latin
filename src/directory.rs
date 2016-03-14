use std::path::{Path, PathBuf};
use std::fs::{remove_dir_all, read_dir};
use std::io::Result as IoResult;

/// Returns true if `path` exists and is a directory.
///
/// ```rust,no_run
/// if latin::directory::exists("/tmp") {
///     // do stuff
/// }
pub fn exists<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    path.is_dir() && path.exists()
}

/// Returns an iterator of the children in `path`.
///
/// ```rust,no_run
/// for child in latin::directory::children("./").unwrap() {
///     println!("{:?}", child);
/// }
pub fn children<P: AsRef<Path>>(path: P) -> IoResult<::std::vec::IntoIter<PathBuf>> {
    let mut out = vec![];
    for entry in try!(read_dir(path)) {
        let entry = try!(entry);
        out.push(entry.path());
    }
    Ok(out.into_iter())
}

/// Returns an iterator of the child files in `path`.
///
/// ```rust,no_run
/// for file in latin::directory::files("./").unwrap() {
///     for (i, line) in latin::file::read_lines(file).unwrap().enumerate() {
///         println!("{} {}", i, line.unwrap());
///     }
/// }
/// ```
pub fn files<P: AsRef<Path>>(path: P) -> IoResult<::std::vec::IntoIter<PathBuf>> {
    let mut out = vec![];
    for entry in try!(read_dir(path)) {
        let entry = try!(entry);

        // TODO: also check symlinks
        if try!(entry.file_type()).is_file() { 
            out.push(entry.path());
        }
    }
    Ok(out.into_iter())
}

/// Returns a list of all the subdirectories in `path`.
///
/// ```rust,no_run
/// for subdir in latin::directory::sub_directories("./").unwrap() {
///     let children_count = latin::directory::children(subdir).unwrap().count();
///     println!("{:?}: {}", subdir, children_count);
/// }
pub fn sub_directories<P: AsRef<Path>>(path: P) -> IoResult<::std::vec::IntoIter<PathBuf>> {
    let mut out = vec![];
    for entry in try!(read_dir(path)) {
        let entry = try!(entry);

        // TODO: also check symlinks
        if try!(entry.file_type()).is_dir() {
            out.push(entry.path());
        }
    }
    Ok(out.into_iter())
}

/// Removes the directory and all containing files and directories.
///
/// ```rust,no_run
/// latin::directory::remove("/tmp/foobar");
/// ```
pub fn remove<P: AsRef<Path>>(path: P) -> IoResult<()> {
    remove_dir_all(path)
}
