use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(short = 'i', long)]
    pub stdin: bool,

    #[arg(short = 'p', long)]
    pub path: Option<String>,

    #[arg(long)]
    pub filter_extension: Option<String>,

    #[arg(long)]
    pub command: Option<String>,
}
