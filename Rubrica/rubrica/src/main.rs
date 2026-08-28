enum TipoContatto{
    Personale,
    Lavoro,
    Famiglia,
}

struct Contatto {
    nome: String,
    telefono: String,
    eta: u32,
    tipo: TipoContatto,
}

impl Contatto{
    fn new (nome: String, telefono: String, eta: u32, tipo: TipoContatto) -> Self{
        Contatto{nome, telefono, eta, tipo}
    }

    fn e_maggiorenne (&self) -> bool {
        self.eta > 17
    }

    fn set_tel (&mut self, nt: String){
        self.telefono = nt;
    }
}

fn cerca_per_nome(contatto: &[Contatto], nome: &str) -> Option<usize> {
    for (indice, persona) in contatto.iter().enumerate() {
        if persona.nome == nome {
            return Some(indice);
        }
    }
    None
}

fn valida_contatto(c: &Contatto) -> Result<(), String> {
    if c.telefono.is_empty() {
        return Err("il numero di telefono non può essere vuoto".to_string());
    }
    if c.eta >= 120{
        return Err("Bro stai morendo non puoi essere così vecchio".to_string());
    }
    Ok(())
} 

fn valida_rubrica(rubrica: &[Contatto]) -> Result<(), String>{
    for persona in rubrica{
        valida_contatto(persona)?;
    }
    Ok(())
}



fn main() {
    let mario = Contatto::new("Mario".to_string(), "3331234567".to_string(), 30, TipoContatto::Lavoro);
    let luigi = Contatto::new("Luigi".to_string(), "3339876543".to_string(), 17, TipoContatto::Famiglia);
    let anna = Contatto::new("Anna".to_string(), "".to_string(), 150, TipoContatto::Personale);

    let rubrica = vec![mario, luigi, anna];

    // Test e_maggiorenne
    println!("Mario è maggiorenne? {}", rubrica[0].e_maggiorenne());
    println!("Luigi è maggiorenne? {}", rubrica[1].e_maggiorenne());

    // Test cerca_per_nome
    match cerca_per_nome(&rubrica, "Luigi") {
        Some(i) => println!("Luigi trovato all'indice {}", i),
        None => println!("Luigi non trovato"),
    }
    match cerca_per_nome(&rubrica, "Giovanni") {
        Some(i) => println!("Giovanni trovato all'indice {}", i),
        None => println!("Giovanni non trovato"),
    }

    // Test valida_contatto sui singoli
    for persona in &rubrica {
        match valida_contatto(persona) {
            Ok(_) => println!("{}: valido", persona.nome),
            Err(e) => println!("{}: NON valido -> {}", persona.nome, e),
        }
    }

    // Test valida_rubrica (dovrebbe fallire, perché Anna ha dati non validi)
    match valida_rubrica(&rubrica) {
        Ok(_) => println!("Rubrica valida"),
        Err(e) => println!("Rubrica NON valida: {}", e),
    }
}