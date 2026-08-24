mod cli;
mod csv;

#[derive(Debug, Clone)]
pub enum Field {
    Int(i64),
    Float(f64),
    Text(String),
}

#[derive(Debug)]
pub struct Header {
    pub columns: Vec<String>,
}

#[derive(Debug)]
pub struct Row {
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Csv {
    pub header: Header,
    pub rows: Vec<Row>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("Arguments: {:?}", args);
}
