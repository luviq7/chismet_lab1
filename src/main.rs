use std::io;
mod functions;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    let a: f64 = input.trim().parse().unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let b: f64 = input.trim().parse().unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let eps: f64 = input.trim().parse().unwrap();

    println!("Метод дихотомії");
    functions::dix(0, a, b, eps);
    
    println!("Метод хорд");
    functions::hord(0, a, b, eps);
}
