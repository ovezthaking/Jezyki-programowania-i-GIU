/*
4. Wzbogać swoje rozwiązanie o obsługę błędów za pomocą Result.
*/

use std::f64::consts::PI;
use Fig::*;
use std::fmt;
use std::fs::File;
use std::io::Write;
//use std::io::Read;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Debug)]
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

fn save(filename: &str, v: Vec<Fig>) -> io::Result<()>{
    let mut file = File::create(filename)?;
    for fig in v {
        let line = match fig {
            Koło { r } => format!("Koło {}\n", r),
            Prost { a, b } => format!("Prostokąt {} {}\n", a, b),
            Kwadr { a } => format!("Kwadrat {}\n", a),
            Romb { a, alfa } => format!("Romb {} {}\n", a, alfa),
        };
        file.write_all(line.as_bytes())?;
    }
    Ok(())
}



fn load(filename: &str) -> io::Result<Vec<Fig>> {
    let path = Path::new(filename);
    let file = File::open(&path).expect("Nie można otworzyć pliku");
    let reader = io::BufReader::new(file);
    let mut figures = Vec::new();
    
    for line in reader.lines() {
        let line = line.expect("Błąd(line)");
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.is_empty() {
            continue;
        }
        
        let fig = match parts[0] {
            "Koło" => Koło { r: parts[1].parse().expect("Błąd z kółkiem") },
            "Prostokąt" => Prost { a: parts[1].parse().expect("Błąd z prostem"), b: parts[2].parse().expect("Błąd parsowania b") },
            "Kwadrat" => Kwadr { a: parts[1].parse().expect("Błąd z kwadrem") },
            "Romb" => Romb { a: parts[1].parse().expect("Błąd z rombem"), alfa: parts[2].parse().expect("Błąd parsowania alfa") },
            _ => continue,
        };
        
        figures.push(fig);
    }
    Ok(figures)
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

fn main() -> io::Result<()>{
   let mut figury = [Koło { r: 1.5 }, Prost { a: 1.0, b: 2.0 },
   Kwadr { a: 5.0 }, Romb { a: 3.0, alfa : PI / 3.0},
   ];

   for mut f in &mut figury {
       println!("{f} ma pole = {}, obwód = {}", pole(f), obwód(f));
       obrót90(&mut f);
       println!(" Po obrocie {f:?}\n");
   }



    let figury = figury.to_vec();

    println!("\nfigury: {:?}", &figury);

    let _ = save("figury.txt", figury);


    let figury1 = load("figury.txt");

    
    println!("\nfigury1: {:?}", figury1);
   //println!("\n\n{:?}",format!("{:?}", figury));
    Ok(())
}
