/*
fn dividi(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn primo_pari(numeri: &[i32]) -> Option<i32> {
    for &numero in numeri {
        if numero % 2 == 0 {
            return Some(numero);
        }
    }
    None
}

fn main() {
    let risultato = dividi(10.0, 0.0);
    let primo_pari = primo_pari(&[1, 3, 5, 6, 7]);

    match risultato {
        Some(v) => println!("Risultato: {}", v),
        None => println!("Impossibile dividere per zero"),
    }

    match primo_pari {
        Some(v) => println!("Primo numero pari: {}", v),
        None => println!("Nessun numero pari trovato"),
    }
}
*/

fn dividi(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Divisione per zero non permessa".to_string())
    } else {
        Ok(a / b)
    }
}

fn calcola(a: f64, b: f64, c: f64) -> Result<f64, String> {
    let primo = dividi(a, b)?;
    let secondo = dividi(primo, c)?;
    Ok(secondo * 2.0)
}

fn radice_quadrata(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        Err("Impossibile calcolare la radice quadrata di un numero negativo".to_string())
    } else {
        Ok(x.sqrt())
    }
}

fn somma_radici_quadrate(a: f64, b: f64) -> Result<f64, String> {
    let radice_a = radice_quadrata(a)?;
    let radice_b = radice_quadrata(b)?;
    Ok(radice_a + radice_b)
}
fn main() {
    match calcola(100.0, 5.0, 2.0) {
        Ok(v) => println!("Risultato: {}", v),
        Err(e) => println!("Errore: {}", e),
    }

    match calcola(100.0, 0.0, 2.0) {
        Ok(v) => println!("Risultato: {}", v),
        Err(e) => println!("Errore: {}", e),
    }

    match radice_quadrata(16.0) {
        Ok(v) => println!("Radice quadrata: {}", v),
        Err(e) => println!("Errore: {}", e),
    }

    match radice_quadrata(-4.0) {
        Ok(v) => println!("Radice quadrata: {}", v),
        Err(e) => println!("Errore: {}", e),
    }

    match somma_radici_quadrate(9.0, 16.0) {
        Ok(v) => println!("Somma delle radici quadrate: {}", v),
        Err(e) => println!("Errore: {}", e),
    }

    match somma_radici_quadrate(9.0, -16.0) {
        Ok(v) => println!("Somma delle radici quadrate: {}", v),
        Err(e) => println!("Errore: {}", e),
    }
}