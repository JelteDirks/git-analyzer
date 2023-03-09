use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(short = 'p', long)]
    pub path: Option<String>,

    #[arg(short = 'e', long)]
    pub exclude: Option<String>,

    #[arg(short = 'i', long)]
    pub include: Option<String>,

    #[arg(long)]
    pub flags: Option<String>,

    #[arg(short = 'd', long)]
    pub depth: Option<u32>,
}
