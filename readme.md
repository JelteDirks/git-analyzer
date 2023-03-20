# Git Analyzer

I created this project to learn more about the Rust. While making this little
cli, I am reading the second edition of 'The Rust Programming Language' as well
as 'Rust for Rustaceons' and making changes as I go along. For this reason I 
will not be documenting very elaborately. If someone somehow finds some genuine
interest, I might do a better job at that. The documentation might not be up-to-date
for aforementioned reasons.

The help message might already give you some hints hints, and probably saves
many of you some reading time if you actually want to read this.
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

