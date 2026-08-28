mod cli;
mod csv;
mod aggregator;
mod analysis;

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

    let parsed_csv = match csv::parse_csv(&content) {
        Ok(c) => c,
        Err(errors) => {
            eprintln!("Trovati {} errori durante il parsing:", errors.len());
            for e in &errors {
                eprintln!("  riga {}: {}", e.line, e.message);
            }
            std::process::exit(1);
        }
    };

    // column è obbligatoria per tutte le modalità tranne count
    if args.mode != "count" && args.column.is_none() {
        eprintln!("Errore: --column è obbligatorio per la modalità '{}'", args.mode);
        std::process::exit(1);
    }

    let mut aggregator = match analysis::make_aggregator(&args.mode) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Errore: {}", e);
            std::process::exit(1);
        }
    };

    let filter = match analysis::make_filter(args.filter.as_deref(), &parsed_csv.header.columns) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Errore: {}", e);
            std::process::exit(1);
        }
    };

    let col_index = if let Some(col_name) = &args.column {
        match parsed_csv.header.columns.iter().position(|c| c == col_name) {
            Some(idx) => Some(idx),
            None => {
                eprintln!("Errore: colonna '{}' non trovata", col_name);
                std::process::exit(1);
            }
        }
    } else {
        None
    };

    let filtered_rows: Vec<&csv::Row> = parsed_csv
        .rows
        .iter()
        .filter(|row| filter(row))
        .collect();

    let rows_analyzed = filtered_rows.len();

    for row in &filtered_rows {
        let value = match col_index {
            Some(idx) => csv::field_to_f64(&row.fields[idx]),
            None => 0.0, // count non usa il valore
        };
        aggregator.update(value);
    }

    println!("mode: {}", args.mode);
    if let Some(col_name) = &args.column {
        println!("column: {}", col_name);
    }
    if let Some(filter_expr) = &args.filter {
        println!("filter: {}", filter_expr);
    }
    println!("result: {}", aggregator.result());
    println!("rows_analyzed: {}", rows_analyzed);
}