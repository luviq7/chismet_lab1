mod functions;
fn main() {
    let a: f64 = 6.0;
    let b: f64 = 7.0;
    let eps: f64 = 0.00001;
    println!("Метод дихотомії");
    functions::dix(0, a, b, eps);
    println!("Метод хорд");
    functions::hord(0, a, b, eps);

}
