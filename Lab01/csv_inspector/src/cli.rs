#[derive(Debug)]
pub struct CliArgs {
    pub file_path: String,
    pub head: usize,
}

pub fn parse_args(args: &[String]) -> Result<CliArgs, String> {
    if args.is_empty() {
        return Err("No arguments provided".to_string());
    }

    let file_path = args[0].clone();

    let pos: Option<usize> = args.iter().position(|a| a == "--head");

    let head: usize = match pos {
        Some(pos) => {
            let value = args.get(pos + 1).ok_or("Missing value after --head".to_string())?;
            value.parse::<usize>().map_err(|_| "Invalid head value".to_string())?
        }
        None => 10,
    };

    Ok(CliArgs { file_path, head })
}