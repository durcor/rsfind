# rsfind
Recursively search for files with a variety of things in the given directory.

# Why?
Too many tools exist to extract simple information from files.
Solutions like `fgrep` and `find` have archaic syntaxes which don't adhere to traditional human thought when it comes to sifting through files for what you need.
It is time we provide developers, systems administrators, and power users with the ability to quickly query their files for the information they deserve.
Let me introduce you to `rsfind` (because it finds info on files, but in Rust).

## Building
```sh
# Debug build
cargo build
# Optimized, symbol-stripped build
cargo build --release
```

## Usage
```sh
rsfind [permissions, name] [of, with] <query> in <directory>
```

## Semantics
- `name`
    - of: searches for files with the exact name of the query
    - with: searched for files with a name containing the substring of the query
- `permissions`
    - Some acceptable permission strings
        - rwxrwxrwx
        - rw-r--r--

## Some Ideas for the Future
- fgrep replacement: `rsfind {contents, data} with <query> in <directory>`
- largest file/directory: `rsfind max in <directory>`
- files/directories larger than a size: `rsfind size {greater, less} than <query> in <directory>`
- modification/creation date: `rsfind {newer, older} than <query> in <directory>`
