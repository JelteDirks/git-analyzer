use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(long)]
    stdin: bool,

    #[arg(long)]
    path: String,
}
