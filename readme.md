# Latin

### A simplified and extended standard library

> Simple things should be simple, complex things should be possible. -- Alan Kay

Rust's standard library is an impressive feat of engineering.  It manages to
stay consistent, performant, and extendable.  However, performing simple tasks
and remaining general are often at odds.  Take for example the task of writing
some text to a file.  With the rust stdlib an implementation would look like
this:

```rust
use std::io::Write;

let mut file = try!(std::fs::File::Create(FILE_NAME));
try!(file.write_all(CONTENTS));
std::mem::drop(file);
```

All the verbosity comes from the rust standard-library's goal of staying
general; however, readability takes a hit.

Latin attempts to take as many common operations and make them as clear
and easy-to-remember as possible.  The same program as above written
with Latin would be

```rust
try!(latin::file::write(FILE_NAME, CONTENT));
```

### Ideology

* Simple things should be simple.
    * Identify overly verbose code that is regularly re-written and make it a
      one-liner.
* Preserve errors.
    * Latin should expose Results as they are in the
  code that they replace.
* Expand beyond just found in the rust standard library.
    * Downloading files, parsing/saving images, are all on the roadmap.
