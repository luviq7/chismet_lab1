pub fn prnt(n: i64, a: f64, b: f64, absol: f64, eps: f64, x: f64){
    if n == 0 {
        println!("{:<5}{:<14}{:<14}{:<16}{:<14}{:<16}", "N", "a", "b", "|expected eps|", "eps", "x");
    }
    println!("{:<5}{:<14.5}{:<14.5}{:<16.5}{:<14.5}{:<16.5}", n, a, b, absol, eps, x);
}

pub fn dix(n: i64, mut a: f64, mut b: f64, eps: f64) -> f64{
    let x: f64 = (a + b) / 2f64;
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
        prnt(n, a, b, (a-b).abs(),eps, x);
        return dix(n+1, a, b, eps)
    }
}
pub fn hord(n: i64, a: f64, b: f64, eps: f64) -> f64{
    let x: f64 = a - (func(a)*(b-a))/(func(b)-func(a));
    if (a-x).abs()<eps || func(x) == 0.0{
        return x
    }
    else{
        prnt(n, a, b, (a-x).abs(),eps, x);
        return hord(n+1, x, b, eps)
    }
}

pub fn func(x: f64) -> f64{
    x.powi(3) - 6f64 * x.powi(2) - 7f64
}