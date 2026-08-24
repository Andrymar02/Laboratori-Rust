mod cli;
mod io;

fn main() {
    let argst: Vec<String> = std::env::args().collect();
    let cli_args_input = &argst[1..];

    let cli_args = match cli::parse_args(cli_args_input) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match io::read_and_print(&cli_args.file_path, cli_args.head) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
}
