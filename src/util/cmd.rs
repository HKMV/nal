use clap_derive::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'i')]
    pub install: bool,

    #[arg(short,long)]
    pub uninstall: bool,

    #[arg(long)]
    pub start: bool,

    #[arg(long)]
    pub stop: bool,

    #[arg(short,long)]
    pub run: bool
}
