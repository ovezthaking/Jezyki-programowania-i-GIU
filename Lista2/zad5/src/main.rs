/*
5. Dla klasy struct Frac(i32,i32) zaimplementuj traity Add, Sub, Mul, Div, oraz dodaj
#[derive(Debug)] oraz metodę fn uprosc(&mut self)->Frac, aby działało:
fn main(){
let a=Frac(2,3); let b=Frac(2,4); let c=Frac(2,3); let d=(a+b-c)*b/c;
println!("Wynik: {:?}",d.uprosc());
}
*/

use::std::*;
use::std::ops::{Add, Mul, Sub, Div};

/*

2/3 + 4/5 = 2*5/3*5 + 4*3/5*3 = 22/15
2/3 - 4/5 = 2*5/3*5 - 4*3/5*3 = 10/15 - 12/15 = -2/15
2/3 * 4/5 = 2*4/3*5 = 8/15
2/3 : 4/5 = 2/3 * 5/4 = 10/12

*/



#[derive(Debug, Clone, Copy)]

struct Frac(i32, i32);

impl Frac {
    fn nwd(mut a:i32, mut b:i32) -> i32{
        let result = loop{
            let r = a%b;
            a = b;
            b = r;
            if r == 0{
                break a;
            }
        };
        result
    }
    
    fn uprosc(&mut self) -> Frac {
        
        let mut nwd = Self::nwd(self.0, self.1);
        if nwd < 0{
            nwd = -nwd;
        }
        self.1 /= nwd;
        self.0 /= nwd;

        *self
    }

}

impl Add for Frac{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0*other.1 + other.0*self.1, self.1*other.1)
    }
}

impl Sub for Frac{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0*other.1 - other.0*self.1, self.1*other.1)
    }
}

impl Mul for Frac {
    type Output = Self;

    fn mul(self, other:Self) -> Self {
        Self(self.0*other.0, self.1*other.1)
    }
}

impl Div for Frac {
    type Output = Self;

    fn div(self, other:Self) -> Self {
        Self(self.0*other.1, self.1*other.0)
    }
}


fn main(){
    let a=Frac(2,3); 
    let b=Frac(2,4); 
    let c=Frac(2,3); 
    let mut d=(a+b-c)*b/c;
    //let mut d = a.clone() + b + a;
    //let mut d=a-b;
    //print!("{:?}", d);
    println!("Wynik: {:?}",d.uprosc());
    //println!("Wynik: {:?}",d.uprosc());
}