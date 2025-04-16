
use std::{ops::{Add, Mul, Sub}, vec};
use std::fmt;

#[derive(Clone)]
struct Polyf32{
    a: Vec<f32> 
}


#[derive(Clone)]
struct Poly<T>{
    a: Vec<T>
}


impl<T> Poly<T>
where
    T: Into<f64> + From<f64> + Copy,
{
    fn eval(&self, x: T) -> T {
        let mut sum = 0.0;
        for (i, &coeff) in self.a.iter().enumerate() {
            sum += coeff.into() * x.into().powi(i as i32);
        }
        T::from(sum) 
    }
}

/* 
impl<T> Add for Poly<T>
where
    T: Add<Output = T> + From<f64>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut output_wek;
        if self.a.len() >= other.a.len() {
            output_wek = vec![0.0; self.a.len()];
            for i in 0..self.a.len() {
                output_wek[i] += self.a[i];
            }
            for i in 0..other.a.len() {
                output_wek[i] += other.a[i];
            }
        } else {
            output_wek = vec![0.0; other.a.len()];
            for i in 0..other.a.len() {
                output_wek[i] += other.a[i];
            }
            for i in 0..self.a.len() {
                output_wek[i] += self.a[i];
            }
        }
        Poly {
            a: output_wek,
        }
    }
}

*/

impl fmt::Display for Polyf32{
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
                /* 
                if self.a[v] == 0.0{
                    write!(f, "")?;
                }*/
                else{
                    write!(f,"")?;
                }
            }
        }
        Ok(())
    }
}


impl Polyf32{
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
    fn print_eval(&self, x:f32){
        println!("Wartość wielomianu w punkcie {} wynosi: {}", x, self.eval(x));
    }
}



impl Add for Polyf32{
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
        Polyf32 {a: output_wek}
    }
    
}


impl Sub for Polyf32{
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
        Polyf32 {a: output_vec}
    }
}


impl Mul for Polyf32{
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
        Polyf32 {a: output_vec}
    }
}


fn main() {
    let wiel1 = Polyf32{a: vec![1.0, 2.0]};
    let wiel2 = Polyf32{a: vec![4.0, -3.0, 0.0 ,-1.0]};

    let wiel = Poly{a: vec![1.0, 2.0]};

    let p: Poly<f32> = Poly{a: vec![1.0, 2.0, 3.0]};
    let q: Poly<f32> = Poly{a: vec![4.0, -3.0, 0.0, -1.0]};

    

    

    let x: f32 = 2.0;
    let eval = p.eval(q.eval(x));
    println!("Generic EVAL: {}", eval);

    let x1 = 2.0;
    let x2 = 4.0;

    wiel1.print_eval(x1);
    wiel2.print_eval(x2);

    println!("\nW1(x) = {}", &wiel1);
    println!{"W2(x) = {}", &wiel2};

    println!("\nW1(x) + W2(x) = {}", wiel1.clone() + wiel2.clone());

    //let wieltest = wiel1.clone() - wiel2.clone();
    println!("\nW1(x) - W2(x) = {}", wiel1.clone() - wiel2.clone());
    //print!("test = {:?}", wieltest.a);
    println!("W2(x) - W1(x) = {}", wiel2.clone() - wiel1.clone());

    println!("\nW1(x) * W2(x) = {}", wiel1 * wiel2);

}
