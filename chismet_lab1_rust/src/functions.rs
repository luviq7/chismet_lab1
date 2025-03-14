/*
pub fn recurs(n: i64) -> i64{
    1
}
*/

pub fn dix(n: i64, mut a: f64, mut b: f64, eps: f64) -> f64{
    let x: f64 = (a + b) / 2f64;
    if n == 0{
        println!("Метод дихотомії");
        println!("{:<5}{:<14}{:<14}{:<16}{:<14}{:<16}", "N", "a", "b", "|a-b|", "eps", "x");
        println!("{}", "-".repeat(74));
        println!("{:<5}{:<14.8}{:<14.8}{:<16.8}{:<14.8}{:<16.8}", n, a, b, (a - b).abs(), eps, x);
    }
    if (a-b).abs() < eps || func(x) == 0.0{
        return x
    }
    else{
        if func(a) * func(x) > 0.0{
            a = x;
        }
        if func(b) * func(x) > 0.0{
            b = x;
        }
        if n > 0 {
            println!("{:<5}{:<14.8}{:<14.8}{:<16.8}{:<14.8}{:<16.8}", n, a, b, (a - b).abs(), eps, x);
        }
        return dix(n+1, a, b, eps)
    }
}
pub fn hord(n: i64, a: f64, b: f64, eps: f64) -> f64{
    let x: f64 = a - (func(a)*(b-a))/(func(b)-func(a));
    if n == 0{
        println!("Метод хорд");
        println!("{:<5}{:<14}{:<14}{:<16}{:<14}{:<16}", "N", "a", "b", "|a-x|", "eps", "x");
        println!("{}", "-".repeat(74));
        println!("{:<5}{:<14.8}{:<14.8}{:<16.8}{:<14.8}{:<16.8}", n, a, b, (a - b).abs(), eps, x);
    }
    if (a-x).abs()<eps || func(x) == 0.0{
        return x
    }
    else{
        if n > 0{
            println!("{:<5}{:<14.8}{:<14.8}{:<16.8}{:<14.8}{:<16.8}", n, a, b, (a - x).abs(), eps, x);
        }
        return hord(n+1, x, b, eps)
    }
}


pub fn func(x: f64) -> f64{
    x.powi(3) - 6f64 * x.powi(2) - 7f64
}