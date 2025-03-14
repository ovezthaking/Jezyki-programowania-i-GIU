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

Dobra to myślę, że taki wektor chyba musi być w formie [...,a,b,c], gdzie W(x) = ... + x^2 * a + x^1 * b + x * c


A może tak: W(x) = a* x^0 + b * x^1 + c * x^2 ...
i niech forma będzie [a,b,c,...]
*/
use std::ops::{Add, Mul, Sub};


#[derive(Clone)]
struct Poly{
    a: Vec<f32> //wektor współczynników wielomianu
}

impl Poly{
    fn eval(&self, x:f32) -> f32{
        let mut sum = 0.0;
        let wek = &self.a;
        for i in 0..wek.len() {
            //println!("{}", f32::powi(x, (i as i32).try_into().unwrap()));
            sum += wek[i] * f32::powi(x, (i as i32).try_into().unwrap());
            //println!("{} * {} ^ {} = {}", &wek[i], &x, &i, &sum);
        }

        sum
    }
}

fn main() {
    let wiel1 = Poly{a: vec![1.0, 3.0, 4.0]};
    //println!("{:?}", wiel1.a);
    //let wek = [2,4,6,2,4,7];
    //let wek = wiel.a();
    //let sum:f32;
    /* 
    let wek = wiel1.a;
    let x = 2.0;
    let mut sum = 0.0;
    for i in (0..wek.len()) {
        //println!("{}", f32::powi(x, (i as i32).try_into().unwrap()));
        sum += wek[i] * f32::powi(x, (i as i32).try_into().unwrap());
        println!("{} * {} ^ {} = {}", &wek[i], &x, &i, &sum);
        
    }
    println!("\n{}", sum);
    */

    println!("{}", &wiel1.eval(2.0));


}



/*

*/



/*
for i = 0 in wektor_wielomianow{
    x += 
}

*/