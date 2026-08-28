/*fn main() {
    let numeri = vec![1, 2, 3, 4, 5];

    let doppi: Vec<i32> = numeri.iter().map(|n| n * 2).collect();

    println!("{:?}", doppi);
}*/

/*fn main() {
    let numeri = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let pari: Vec<&i32> = numeri.iter().filter(|n| **n % 2 == 0).collect();

    println!("{:?}", pari);
}*/

/*fn main() {
    let numeri = vec![1, 2, 3, 4, 5];

    let somma = numeri.iter().fold(0, |accumulatore, n| accumulatore + n);

    println!("Somma: {}", somma);
}*/

fn main() {
    let numeri = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let somma = numeri.iter().cloned().filter(|n| n%2 == 0).fold(0, |accumulatore, n| accumulatore + n * n);

    println!("Somma dei guadrati pari: {}", somma);
}