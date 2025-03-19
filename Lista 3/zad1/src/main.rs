/*
Zaimplementuj dla wielomianów trait std::fmt::Display.

Dobra to myślę, że taki wektor chyba musi być w formie [...,a,b,c], gdzie W(x) = ... + x^2 * a + x^1 * b + x * c


A może tak: W(x) = a* x^0 + b * x^1 + c * x^2 ...
i niech forma będzie [a,b,c,...]


Lista 3 zad 1 
Trzeba zrobić From i przekonwertować to tak żrby dało sie zrobic w np f64 , chyba From<f64> 
*/
use std::{ops::{Add, Mul, Sub}, vec};
use std::fmt;

#[derive(Clone)]
struct Poly<T>{
    a: Vec<T> //wektor współczynników wielomianu
}


impl fmt::Display for Poly<f32>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        for v in (0..self.a.len()).rev(){
            if v == 0 {
                if self.a[v] < 0.0 {
                    write!(f, "- {} ", &self.a[v]*-1.0)?;
                }
                if self.a[v] > 0.0 {
                    write!(f, "{} ", &self.a[v])?;
                }
                else{
                    write!(f,"")?;
                }
            }
            if v != 0 {
                if self.a[v] < 0.0 {
                    if self.a[v-1] <= 0.0{
                        write!(f, "- {} * x^{} ", &self.a[v] * -1.0,&v)?;
                    }
                    else{
                        write!(f, "- {} * x^{} + ", &self.a[v] * -1.0,&v)?;
                    }
                }
                if self.a[v] > 0.0 {
                    if self.a[v-1] < 0.0{
                        write!(f, "{} * x^{} ", &self.a[v],&v)?;
                    }
                    else {
                        write!(f, "{} * x^{} + ", &self.a[v],&v)?;
                    }
                }
                else{
                    write!(f,"")?;
                }
            }
        }
        Ok(())
    }
}


impl<T> Poly<T>
where
    T: Copy + Into<f64> + From<f64> + std::ops::AddAssign + std::ops::Mul<Output = T>,
{
    fn eval(&self, x: T) -> T {
        let mut sum = T::from(0.0);
        let wek = &self.a;
        for (i, &coeff) in wek.iter().enumerate() {
            sum += coeff * T::from(x.into().powi(i as i32));
        }
        sum
    }
    fn print_eval(&self, x: T) {
        println!(
            "Wartość wielomianu w punkcie {} wynosi: {}",
            x.into(),
            self.eval(x).into()
        );
    }
}



impl Add for Poly<f32>{
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


impl Sub for Poly<f32>{
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


impl Mul for Poly<f32>{
    type Output = Self;

    fn mul(self, other:Self) -> Self{
        let mut output_vec = vec![0.0; self.a.len() + other.a.len() - 1];
        for i in 0..self.a.len(){
            for j in 0..other.a.len(){
                output_vec[i+j] += self.a[i] * other.a[j];
            }
        }
        Poly {a: output_vec}
    }
}


fn main() {
    let wiel1 = Poly{a: vec![1.0, 2.0]};
    let wiel2 = Poly{a: vec![4.0, -3.0, 0.0 ,-1.0]};

    let x1 = 2.0;
    let x2 = 4.0;

    
    //wiel1.print_eval(x1);
    //wiel2.print_eval(x2);


    println!("\nW1(x) = {}", &wiel1);
    println!{"W2(x) = {}", &wiel2};

    println!("\nW1(x) + W2(x) = {}", wiel1.clone() + wiel2.clone());

    println!("\nW1(x) - W2(x) = {}", wiel1.clone() - wiel2.clone());
    
    println!("W2(x) - W1(x) = {}", wiel2.clone() - wiel1.clone());

    println!("\nW1(x) * W2(x) = {}", wiel1 * wiel2);

}