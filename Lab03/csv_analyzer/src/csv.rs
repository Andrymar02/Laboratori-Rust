use std::fmt;

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

#[derive(Debug)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Field::Int(i) => write!(f, "{}", i),
            Field::Float(v) => write!(f, "{}", v),
            Field::Text(s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Display for Csv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.header.columns.join(" | "))?;

        let separator: Vec<String> = self
            .header
            .columns
            .iter()
            .map(|c| "-".repeat(c.len()))
            .collect();
        writeln!(f, "{}", separator.join("-+-"))?;

        for row in &self.rows {
            let values: Vec<String> = row.fields.iter().map(|field| field.to_string()).collect();
            writeln!(f, "{}", values.join(" | "))?;
        }

        Ok(())
    }
}

// Interpreta una singola stringa come Field: prova Int, poi Float, altrimenti Text
fn parse_field(raw: &str) -> Field {
    if let Ok(i) = raw.parse::<i64>() {
        Field::Int(i)
    } else if let Ok(f) = raw.parse::<f64>() {
        Field::Float(f)
    } else {
        Field::Text(raw.to_string())
    }
}

// Confronta il "tipo" di due Field, ignorando il valore contenuto
fn same_type(a: &Field, b: &Field) -> bool {
    matches!(
        (a, b),
        (Field::Int(_), Field::Int(_))
            | (Field::Float(_), Field::Float(_))
            | (Field::Text(_), Field::Text(_))
    )
}

fn type_name(f: &Field) -> &'static str {
    match f {
        Field::Int(_) => "Intero",
        Field::Float(_) => "Decimale",
        Field::Text(_) => "Testo",
    }
}

pub fn parse_csv(content: &str) -> Result<Csv, Vec<ParseError>> {
    let mut lines = content.lines();

    let header_line = match lines.next() {
        Some(l) => l,
        None => {
            return Err(vec![ParseError {
                line: 0,
                message: "File vuoto: manca l'header".to_string(),
            }])
        }
    };

    let header = Header {
        columns: header_line.split(',').map(|s| s.to_string()).collect(),
    };

    let mut rows: Vec<Row> = Vec::new();
    let mut errors: Vec<ParseError> = Vec::new();
    let mut expected_types: Option<Vec<Field>> = None;

    for (i, line) in lines.enumerate() {
        let line_number = i + 2; // riga 1 è l'header, quindi i dati partono da riga 2
        let raw_fields: Vec<&str> = line.split(',').collect();

        if raw_fields.len() != header.columns.len() {
            errors.push(ParseError {
                line: line_number,
                message: format!(
                    "Numero di campi errato: attesi {}, trovati {}",
                    header.columns.len(),
                    raw_fields.len()
                ),
            });
            continue;
        }

        let parsed_fields: Vec<Field> = raw_fields.iter().map(|s| parse_field(s)).collect();

        match &expected_types {
            None => {
                // Prima riga di dati: stabilisce i tipi attesi per le colonne successive
                expected_types = Some(parsed_fields.clone());
                rows.push(Row { fields: parsed_fields });
            }
            Some(expected) => {
                let mut row_ok = true;
                for (col_index, (field, expected_field)) in
                    parsed_fields.iter().zip(expected.iter()).enumerate()
                {
                    if !same_type(field, expected_field) {
                        let col_name = &header.columns[col_index];
                        errors.push(ParseError {
                            line: line_number,
                            message: format!(
                                "Colonna '{}': atteso {}, trovato {}",
                                col_name,
                                type_name(expected_field),
                                type_name(field)
                            ),
                        });
                        row_ok = false;
                    }
                }
                if row_ok {
                    rows.push(Row { fields: parsed_fields });
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(Csv { header, rows })
    } else {
        Err(errors)
    }
}

pub fn field_to_f64(field: &Field) -> f64 {
    match field {
        Field::Int(i) => *i as f64,
        Field::Float(f) => *f,
        Field::Text(s) => s.len() as f64,
    }
}