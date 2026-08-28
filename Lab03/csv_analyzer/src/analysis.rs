use anyhow::{Result, anyhow};
use crate::aggregator::{Aggregator, Count, Sum, Average, Min, Max};
use crate::csv::{Field, Row};

pub fn make_aggregator(mode: &str) -> Result<Box<dyn Aggregator>> {
    match mode {
        "count" => Ok(Box::new(Count::new())),
        "sum" => Ok(Box::new(Sum::new())),
        "avg" => Ok(Box::new(Average::new())),
        "min" => Ok(Box::new(Min::new())),
        "max" => Ok(Box::new(Max::new())),
        _ => Err(anyhow!("Modalità non valida: '{}'. Modalità disponibili: count, sum, avg, min, max", mode)),
    }
}

#[derive(Clone)]
enum Operator {
    Eq,
    Gt,
    Lt,
}

fn parse_operator(s: &str) -> Result<Operator> {
    match s {
        "=" => Ok(Operator::Eq),
        ">" => Ok(Operator::Gt),
        "<" => Ok(Operator::Lt),
        _ => Err(anyhow!("Operatore non valido: '{}'", s)),
    }
}

fn split_expression(expression: &str) -> Result<(&str, Operator, &str)> {
    let pos = expression
        .find(|c: char| c == '=' || c == '>' || c == '<')
        .ok_or_else(|| anyhow!("Espressione di filtro malformata: '{}'", expression))?;

    let column = &expression[..pos];
    let op_str = &expression[pos..pos + 1];
    let value = &expression[pos + 1..];

    let op = parse_operator(op_str)?;

    Ok((column, op, value))
}

// Confronta un Field con una stringa di valore, secondo l'operatore.
// Se il campo è testo, confronto testuale. Se è numerico, confronto numerico.
fn field_matches(field: &Field, op: &Operator, value_str: &str) -> bool {
    match field {
        Field::Text(s) => match op {
            Operator::Eq => s == value_str,
            Operator::Gt => s.as_str() > value_str,
            Operator::Lt => s.as_str() < value_str,
        },
        Field::Int(i) => {
            let target = *i as f64;
            compare_numeric(target, op, value_str)
        }
        Field::Float(f) => compare_numeric(*f, op, value_str),
    }
}

fn compare_numeric(field_value: f64, op: &Operator, value_str: &str) -> bool {
    let target = match value_str.parse::<f64>() {
        Ok(v) => v,
        Err(_) => return false,
    };
    match op {
        Operator::Eq => field_value == target,
        Operator::Gt => field_value > target,
        Operator::Lt => field_value < target,
    }
}

pub fn make_filter(
    expression: Option<&str>,
    columns: &[String],
) -> Result<Box<dyn Fn(&Row) -> bool>> {
    match expression {
        None => Ok(Box::new(|_row: &Row| true)),
        Some(expr) => {
            let (column, op, value_str) = split_expression(expr)?;

            let col_index = columns
                .iter()
                .position(|c| c == column)
                .ok_or_else(|| anyhow!("Colonna non trovata: '{}'", column))?;

            let value_str = value_str.to_string();

            Ok(Box::new(move |row: &Row| {
                field_matches(&row.fields[col_index], &op, &value_str)
            }))
        }
    }
}