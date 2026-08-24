use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Parser CSV strutturato")]
pub struct Args {
    /// Percorso del file CSV da leggere
    pub file_path: String,
}