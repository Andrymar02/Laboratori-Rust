enum Categoria {
    Alimentare,
    Elettronica,
    Abbigliamento,
}

struct Prodotto {
    nome: String,
    prezzo: f32,
    quantita: u32,
    categoria: Categoria,
}

impl Prodotto {
    fn new(nome: String, prezzo: f32, quantita: u32, categoria: Categoria) -> Self{
        Prodotto {nome, prezzo, quantita, categoria}
    }

    fn valore_totale(&self) -> f32 {
        self.prezzo * self.quantita as f32
    }

    fn applica_sconto(&mut self, percentuale: f32) {
        self.prezzo -= self.prezzo * (percentuale / 100.0);
    }
}

fn trova_prodotto<'a>(magazzino: &'a [Prodotto], nome: &str) -> Option<&'a Prodotto> {
    for prodotto in magazzino {
        if prodotto.nome == nome {
            return Some(prodotto);
        }
    }
    None
}

fn valida_prodotto(p: &Prodotto) -> Result<(), String> {
    if p.nome.trim().is_empty() {
        return Err("Il nome del prodotto non può essere vuoto.".to_string());
    }
    if p.prezzo < 0.0 {
        return Err("Il prezzo del prodotto non può essere negativo.".to_string());
    }
    if p.quantita == 0 {
        return Err("La quantità del prodotto deve essere maggiore di zero.".to_string());
    }
    Ok(())
}

fn valida_magazzino(magazzino: &[Prodotto]) -> Result<(), String> {
    for prodotto in magazzino {
        valida_prodotto(prodotto)?;
    }
    Ok(())
}

fn main(){
    let mut magazzino: Vec<Prodotto> = Vec::new();

    let prodotto1 = Prodotto::new("Pasta".to_string(), 1.5, 100, Categoria::Alimentare);
    let prodotto2 = Prodotto::new("Laptop".to_string(), 1200.0, 10, Categoria::Elettronica);
    let prodotto3 = Prodotto::new("Maglietta".to_string(), 20.0, 50, Categoria::Abbigliamento);
    let prodotto4 = Prodotto::new("".to_string(), 10.0, 5, Categoria::Abbigliamento); // Prodotto con nome vuoto
    let prodotto5 = Prodotto::new("Telefono".to_string(), -500.0, 5, Categoria::Elettronica); // Prodotto con prezzo negativo

    magazzino.push(prodotto1);
    magazzino.push(prodotto2);
    magazzino.push(prodotto3);
    magazzino.push(prodotto4);
    magazzino.push(prodotto5);

    match valida_magazzino(&magazzino) {
        Ok(_) => println!("Magazzino valido."),
        Err(e) => println!("Errore nella validazione del magazzino: {}", e),
    }

    if let Some(prodotto) = trova_prodotto(&magazzino, "Laptop") {
        println!("Trovato prodotto: {} con valore totale: {}", prodotto.nome, prodotto.valore_totale());
    } else {
        println!("Prodotto non trovato.");
    }
}