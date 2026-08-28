use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Motore di analisi CSV")]
pub struct Args {
    /// Percorso del file CSV da leggere
    pub file_path: String,

    /// Modalità di aggregazione: count, sum, avg, min, max
    #[arg(long)]
    pub mode: String,

    /// Nome della colonna su cui operare (richiesto per sum, avg, min, max)
    #[arg(long)]
    pub column: Option<String>,

    /// Espressione di filtro, es. "età>25"
    #[arg(long)]
    pub filter: Option<String>,
}