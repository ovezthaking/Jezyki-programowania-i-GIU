//2. Dla struktury z poprzedniego zadania zaimplementuj trait std::Display.
use std::f64::consts::PI;
use Fig::*;
use std::fmt;

#[derive(Debug)]
enum Fig {Koło {r: f64 }, Prost {a: f64, b: f64},
       Kwadr {a: f64}, Romb {a: f64, alfa: f64},}


impl fmt::Display for Fig{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Koło {r} => write!(f, "Koło o promieniu {}", r),
            Prost {a, b} => write!(f, "Prostokąt o bokach {} i {}", a, b),
            Kwadr {a} => write!(f, "Kwadrat o boku {}", a),
            Romb {a, alfa} => write!(f, "Romb o boku {} i kącie {}", a, alfa),
        }
    }
}


fn pole(f: &Fig) -> f64 {
   match f {
       Koło {r} => PI * r * r,
       Prost {a, b} => a * b,
       Kwadr {a} => a * a,
       Romb {a, alfa} => a * a * alfa.sin(),
   }
}

fn obwód(f: &Fig) -> f64 {
   match f {
       Koło {r} => 2.0 * PI * r,
       Prost {a, b} => 2.0 * a + 2.0 * b,
       Kwadr {a} => 4.0 * a,
       Romb {a, alfa: _} => 4.0 * a,
   }
}

fn obrót90(f: &mut Fig) {
    match f {
        Koło {r: _} => (),
        Prost {a, b} => {
         let tmp = *a; 
         *a = *b; 
         *b = tmp;
     },
        Kwadr {a: _} => (),
        Romb {a: _, alfa} => {
            *alfa = ((*alfa + PI) / 2.0) % (2.0 * PI);
         }
     }
 }

fn main(){
   let mut figury = [Koło { r: 1.5 }, Prost { a: 1.0, b: 2.0 },
   Kwadr { a: 5.0 }, Romb { a: 3.0, alfa : PI / 3.0},
   ];

   for mut f in &mut figury {
       println!("{f} ma pole = {}, obwód = {}", pole(f), obwód(f));
       obrót90(&mut f);
       println!(" Po obrocie {f:?}\n");
   }
    
   //println!("\n\n{:?}",format!("{:?}", figury));
}
