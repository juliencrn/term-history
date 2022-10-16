use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(about = "Print the most used cli programs", long_about = None)]
pub struct Args {
    pub history_file: PathBuf,

    #[arg(long, default_value = None, help = "Enable colors")]
    pub colors: bool,

    #[arg(long, default_value_t = 20, help = "Print the first n lines")]
    pub head: u8,
}
