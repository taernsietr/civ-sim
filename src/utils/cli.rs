use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = 800)]
    pub x: u32,

    #[arg(short, long, default_value_t = 600)]
    pub y: u32,

    #[arg(short, long, default_value_t = 1)]
    pub variations: u32,
    
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,

    #[arg(short, long)]
    pub file: Option<String>,

    #[arg(short, long)]
    pub seeds: Option<Vec<u32>>,
}
