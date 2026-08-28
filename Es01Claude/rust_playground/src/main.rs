
/* 
Non posso usare s1 dopo averlo spostato in s2, quindi il codice non compila.

fn main() {
    let s1 = String::from("ciao");
    let s2 = s1;
    println!("{}", s1);
}
*/

/* 
funzione clone() crea una copia di s1, quindi posso usare sia s1 che s2, quindi il codice compila.
fn main() {
    let s1 = String::from("ciao");
    let s2 = s1.clone();
    println!("{} {}", s1, s2);
}
*/
/* 
Questo codice compila perché i tipi interi implementano il trait Copy, quindi posso usare sia x1 che x2.
fn main() {
    let x1 = 5;
    let x2 = x1;
    println!("{} {}", x1, x2);
}
*/

/*
Questo codice non compila perché v1 è stato spostato in v2, quindi non posso più usare v1.

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = v1;
    println!("{:?} {:?}", v1, v2);
}
*/

/* 
Questo codice compila perché v2 è un riferimento a v1, quindi posso usare sia v1 che v2. Si parla di Movimento 
fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = &v1;
    println!("{:?} {:?}", v1, v2);
}
*/

/* 
Questo codice compila perchè v1 è un vec mutabiile e v2 è un riferimento mutabile a v1, quindi posso usare sia v1 che v2.
fn main() {
    let mut v1 = vec![1, 2, 3];
    let v2 = &mut v1;
    v2.push(4);
    println!("{:?}", v2);
}
*/

/*
Questo codice non compila perché v2 è un riferimento mutabile a v1, quindi non posso avere un altro riferimento a v1 (v3) mentre v2 è in uso.
fn main() {
    let mut v1 = vec![1, 2, 3];
    let v2 = &mut v1;
    let v3 = &v1;
    v2.push(4);
    println!("{:?}", v2);
}
*/