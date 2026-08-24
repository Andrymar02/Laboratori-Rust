mod cli;
mod csv;

use clap::Parser;

fn main() {
    let args = cli::Args::parse();

    let content = match std::fs::read_to_string(&args.file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Errore nella lettura del file '{}': {}", args.file_path, e);
            std::process::exit(1);
        }
    };

    match csv::parse_csv(&content) {
        Ok(parsed_csv) => {
            println!("{}", parsed_csv);
        }
        Err(errors) => {
            eprintln!("Trovati {} errori durante il parsing:", errors.len());
            for e in &errors {
                eprintln!("  riga {}: {}", e.line, e.message);
            }
            std::process::exit(1);
        }
    }
}