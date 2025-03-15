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
use std::{ops::{Add, Mul, Sub}, vec};


#[derive(Clone)]
struct Poly{
    a: Vec<f32> //wektor współczynników wielomianu
}

impl Poly{
    fn eval(&self, x:f32) -> f32{/* 
        impl Add for Poly{
            type Output = Self;
        
            fn add(self, other: Self) -> Self {
                let max_len = self.a.len().max(other.a.len());
                let mut output_wek = vec![0.0; max_len];
        
                for i in 0..self.a.len() {
                    output_wek[i] += self.a[i];
                }
        
                for i in 0..other.a.len() {
                    output_wek[i] += other.a[i];
                }
        
                Poly { a: output_wek }
            }
        }
        */
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

impl Add for Poly{
    type Output = Self;

    fn add(self, other: Self) -> Self{
        let mut output_wek;
        if self.a.len() >= other.a.len() {
            output_wek = vec![0.0; self.a.len()];
            for i in 0..self.a.len()  {
                output_wek[i] += self.a[i];
            }
            for i in 0..other.a.len(){
                output_wek[i] += other.a[i];
            }
        }
        else{
            output_wek = vec![0.0; other.a.len()];
            for i in 0..other.a.len() {
                output_wek[i] += other.a[i];
            }
            for i in 0..self.a.len(){
                output_wek[i] += self.a[i];
            }
        }
        Poly {a: output_wek}
    }
    
}

impl Sub for Poly{
    type Output = Self;

    fn sub(self, other: Self) -> Self{
        let mut output_vec: Vec<f32>;
        if self.a.len() >= other.a.len(){
            output_vec = vec![0.0; self.a.len()];
            for i in 0..output_vec.len(){
                output_vec[i] += self.a[i];
            }
            for i in 0..other.a.len(){
                output_vec[i] -= other.a[i];
            }
        }
        else{
            output_vec = vec![0.0; other.a.len()];
            for i in 0..self.a.len(){
                output_vec[i] += self.a[i];
            }
            for i in 0..other.a.len(){
                output_vec[i] -= other.a[i];
            }
        }
        Poly {a: output_vec}
    }
}

impl Mul for Poly{
    type Output = Self;

    fn mul(self, other:Self) -> Self{
        let mut output_vec = vec![0.0; self.a.len() + other.a.len() - 1];
        for i in 0..self.a.len(){
            for j in 0..other.a.len(){
                output_vec[i+j] += self.a[i] * other.a[j];
                //println!("{}", i);
                //print!("{}", j);
            }
        }
        Poly {a: output_vec}
    }
}


fn main() {
    let wiel1 = Poly{a: vec![1.0, 2.0]};
    let wiel2 = Poly{a: vec![4.0, -3.0, 1.0]};
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
    println!("W1 = {:?}", &wiel1.a);
    println!("W2 = {:?}", &wiel2.a);

    let wiel3 = wiel1.clone() - wiel2.clone();
    
    println!("{:?}", &wiel3.a);

    let wiel4 = wiel1 * wiel2;
    println!("{:?}", &wiel4.a);



}



/*

*/



/*
for i = 0 in wektor_wielomianow{
    x += 
}

*/


