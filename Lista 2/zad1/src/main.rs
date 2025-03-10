//1. Uzupełnij miejsca wykropkowane tak, aby poniższy kod działał prawidłowo.
 use std::f64::consts::PI;
 use Fig::*;

 #[derive(Debug)]
 enum Fig {Koło {r: f64 }, Prost {a: f64, b: f64},
        Kwadr {a: f64}, Romb {a: f64, alfa: f64},}


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
 
fn obrót90(f: &mut Fig) -> f64 {
    match f {
        Koło {r: _} => 0.0,
        Prost {a, b} => {let tmp = *a; *a = *b; *b = tmp; tmp},
        Kwadr {a: _} => 0.0,
        Romb {a: _, alfa} => {
            *alfa = ((*alfa + PI) / 2.0) % (2.0 * PI);
            ((*alfa + PI) / 2.0) % (2.0 * PI)},
        
    }
}

fn main(){
    let mut figury = [Koło { r: 1.5 }, Prost { a: 1.0, b: 2.0 },
    Kwadr { a: 5.0 }, Romb { a: 3.0, alfa : PI / 3.0},
    ];

    for mut f in &mut figury {
        println!("{f:?} ma pole = {}, obwód = {}", pole(f), obwód(f));
        obrót90(&mut f);
        println!(" Po obrocie {f:?}");
    }

}