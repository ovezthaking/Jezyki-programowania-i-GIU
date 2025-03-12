/*
Zdefiniuj strukturę Poly w następujący sposób:
#[derive(Clone)]
struct Poly {
a:Vec<f32> // wektor współczynników wielomianu
}
Następnie zdefiniuj metodę eval pozwalającą wyliczyć wartość wielomianu w punkcie
x oraz zaimplementuj traity std::ops::Add, std::ops::Mul oraz std::ops::Sub, tak
aby możliwe było dodawanie, odejmowanie i mnożenie wielomianów przez wielomian
oraz przez liczby z lewej i prawej strony.

Dobra to myślę, że taki wektor chyba musi być w formie [...,a,b,c], gdzie W(x) = ... + x^2 * a + xb + c
*/
use std::ops::{Add, Mul, Sub};


#[derive(Clone)]
struct Poly{
    a: Vec<f32> //wektor współczynników wielomianu
}

impl Poly{
    fn eval(&self){
        
    }
}

fn main() {
    println!("Hello, world!");
}
