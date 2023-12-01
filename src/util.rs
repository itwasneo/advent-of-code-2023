use clap::Parser;

#[derive(Parser)]
#[command(version, author, about)]
pub struct Cli {
    #[arg(short, long)]
    pub run: Option<u32>,

    #[arg(short, long)]
    pub measure: Option<u32>,
}
