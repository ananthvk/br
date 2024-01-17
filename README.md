# `br`
A command line tool which helps to rename a lot of files at once using powerful regex substitutions.


## Usage
```
A file utility to rename a lot of files

Usage: br [OPTIONS] <SEARCH> <REPLACE> [DIRECTORY]

Arguments:
  <SEARCH>     Expression for matching files
  <REPLACE>    Replacement expression (can use named groups with ${1})
  [DIRECTORY]  Rename files in the specified directory (default: current directory)

Options:
  -x, --execute               Rename the files, without this flag only a dry run is performed
      --noconfirm             Do not ask any "are you sure?" confirmation questions
  -s, --starts-with <STRING>  Filters files which start with a given pattern. To specify multiple filters -s "foo" -s "bar"
  -h, --help                  Print help
```

## Installation
After installing `Rust`,

```
cargo install --git https://github.com/ananthvk/br
```

## Notes
For substitutions, use `${1}`, `${2}` and so on instead of `\1`

Use single quotes `'` instead  of double quotes to avoid shell expansion when using `${x}` in your replacement string.