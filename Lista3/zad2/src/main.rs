use rug::Integer;


#[derive(Clone)]
struct A (Integer, Integer, Integer);

fn razy(a: A, b: A) -> A {
    let c0 = a.0 * &b.0 + &a.1 * &b.1;
    let c1 = &a.1 * b.0 + &a.2 * &b.1;
    let c2 = a.1 * b.1 + a.2 * b.2;
    A(c0, c1, c2)    
}

fn power(a: A, n: u64) -> A {
    if n == 1 {
        a
    } else if n % 2 == 0 {
        power(razy(a.clone(), a), n / 2)
    } else {
        razy(power(razy(a.clone(), a.clone()), n / 2), a)
    }
}

fn fibo_fast(n: u64) -> Integer {
    if n < 2 {
        return Integer::from(n);
    }
    let A(fib_n, _, _) = power(
        A(Integer::from(1), Integer::from(1), Integer::from(0)),
        n - 1,
    );
    fib_n
}

fn main() {
    let fib = fibo_fast(1_000_000);
    let digits = fib.to_string();
    let pos = 10_000;

    if digits.len() >= pos {
        println!("Cyfra nr 10_000: {}", digits.chars().nth(pos - 1).unwrap());
    } else {
        println!("Fib(1_000_000) ma tylko {} cyfr", digits.len());
    }
    println!("{}", fib);
}
