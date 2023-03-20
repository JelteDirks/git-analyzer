# Git Analyzer

I created this project to learn more about the Rust. While making this little
cli, I am reading the second edition of 'The Rust Programming Language' as well
as 'Rust for Rustaceons' and making changes as I go along. For this reason I 
will not be documenting very elaborately. If someone somehow finds some genuine
interest, I might do a better job at that. The documentation might not be up-to-date
for aforementioned reasons.

The help message might already give you some hints hints, and probably saves
many of you some reading time if you actually want to read this. I won't publish
it to cargo (yet?) so everything is based on running it with cargo.
``` text
Usage: git-analyzer [OPTIONS]

Options:
  -p, --path <PATH>        path to the directory which you want to analyze
  -e, --exclude <EXCLUDE>  extensions that should be excluded, listed without '.' and separated by a space
  -i, --include <INCLUDE>  extensions that should be included, listed without '.' and separated by a space
  -a, --author <AUTHOR>    author of the commits that should be analyzed, passed to git log
  -d, --depth <DEPTH>      exact depth of the directory given by path
  -h, --help               Print help information
  -V, --version            Print version information
```

## path
The path option specifies a directory on the machine to analyze. If only given
a path, the analyzer finds the .git directory in that path and analyzes all objects
in that directory.

Example: you are in a directory which looks like this
```text
./
├── .git/
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── docs/
├── readme.md
├── src/
└── tmp/
```
and you run the following:
```zsh
cargo run -- --path $PWD
```
you will analyze the current directory, and thus the .git folder in the this
directory.

## depth
The depth option specifies an exact depth relative to the given path or default
path. The analyzer recurses down every directory and analyzes only those 
directories that are at the exact depth relative to the given path or default path.

Example:
If you run 
```zsh
cargo run -- --path $PWD --depth 2
```
in a directory that looks like this
```text
./
├── four/
│   ├── .git/
│   └── five/
│       └── .git/
└── one/
    ├── three/
    │   └── .git/
    └── two/
        └── .git/
```
you will analyze the following `.git` directories
```text
./one/two/.git
./one/three/.git
./four/five/.git
```
and NOT `./four/.git/` since --depth is an exact depth.

## exclude
This option excludes extensions from the output. The notation is space separated
so make sure to include the "quotation" marks.

Example
```zsh
cargo run -- --exclude "rs md"
```
excludes `.rs` and `.md` files.

## include
This option includes extensions from the output, and ignores all extensions that
are not in this list. The notation is space separated so make sure to include
"quotation" marks.

Example
```zsh
cargo run -- --include "c rs"
```
includes only `.c` and `.rs` files in the output.
