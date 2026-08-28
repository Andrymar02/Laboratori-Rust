struct Rettangolo {
    larghezza: f64,
    altezza: f64,
}

impl Rettangolo {
    fn nuovo(larghezza: f64, altezza: f64) -> Self {
        Rettangolo { larghezza, altezza }
    }

    fn area(&self) -> f64 {
        self.larghezza * self.altezza
    }

    fn perimetro(&self) -> f64{
        2.0 * self.larghezza + 2.0 * self.altezza
    }

    fn scala(&mut self, fattore: f64) {
        self.larghezza *= fattore;
        self.altezza *= fattore;
    }
}


fn main() {
    let mut r = Rettangolo::nuovo(5.0, 3.0);
    println!("Area: {}", r.area());
    println!("Perimetro: {}", r.perimetro());
    r.scala(2.0);
    println!("Area dopo lo scaling: {}", r.area());
    println!("Perimetro dopo lo scaling: {}", r.perimetro());
}