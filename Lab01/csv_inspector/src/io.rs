pub fn read_and_print(file_path: &str, head: usize) -> Result<(), String> {
    use std::io::{BufRead, BufReader};
    let file = std::fs::File::open(file_path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let mut count = 0;
    println!("head {}: ", head);
    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        if count < head{
            println!("{}", line);
        }
        count += 1;
    }
    println!("Total number of lines: {}", count);
    Ok(())
}