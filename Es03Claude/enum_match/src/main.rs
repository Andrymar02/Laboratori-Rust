/*enum Direzione {
    Nord,
    Sud,
    Est,
    Ovest,
}

fn descrivi(d: Direzione) -> String {
    match d {
        Direzione::Nord => "stai andando verso nord".to_string(),
        Direzione::Sud => "stai andando verso sud".to_string(),
        Direzione::Est => "stai andando verso est".to_string(),
        Direzione::Ovest => "stai andando verso ovest".to_string(),
    }
}

fn main() {
    let d = Direzione::Nord;
    println!("{}", descrivi(d));
}
*/

enum Forma {
    Cerchio(f64),           // contiene il raggio
    Rettangolo(f64, f64),   // contiene larghezza e altezza
    Quadrato(f64),          // contiene il lato
    Triangolo(f64, f64),    // contiene base e altezza
}

fn area(f: &Forma) -> f64 {
    match f {
        Forma::Cerchio(raggio) => std::f64::consts::PI * raggio * raggio,
        Forma::Rettangolo(larghezza, altezza) => larghezza * altezza,
        Forma::Quadrato(lato) => lato * lato,
        Forma::Triangolo(base, altezza) => 0.5 * base * altezza,
    }
}

fn main() {
    let forme = vec![
        Forma::Cerchio(3.0),
        Forma::Rettangolo(4.0, 5.0),
        Forma::Quadrato(2.0),
        Forma::Triangolo(3.0, 4.0),
    ];

    for forma in &forme {
        println!("Area: {}", area(forma));
    }
}