use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// path to the directory which you want to analyze
    #[arg(short = 'p', long)]
    pub path: Option<String>,

    /// extensions that should be excluded, listed without '.' and separated by a space
    #[arg(short = 'e', long)]
    pub exclude: Option<String>,

    /// extensions that should be included, listed without '.' and separated by a space
    #[arg(short = 'i', long)]
    pub include: Option<String>,

    /// author of the commits that should be analyzed, passed to git log
    #[arg(short = 'a', long)]
    pub author: Option<String>,

    /// exact depth of the directory given by path
    #[arg(short = 'd', long)]
    pub depth: Option<u32>,
}
