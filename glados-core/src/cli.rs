use std::path::PathBuf;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   #[arg(short, long)]
   pub ipc_path: PathBuf,
   #[arg(short, long)]
   pub database_url: String,
}