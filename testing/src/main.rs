use core::panic;
use std::ops::Add;

fn main() {
    //1. c)
    for i in Iterator::rev(1..100) {
        if i%2 == 0 {
            print!("{} ", i);
        }
        
    }
    //
    print!("\n\n");
    //
    //1. b)
    let mut i =100;
    loop {
        i -= 1;
        if i%2 == 0 {
            print!("{} ", i);
        }
        if i == 1 {
            break;
        }
    }
    //
    print!("\n\n");
    //
    //1. a)
    i = 100;
    while i != 1 {
        i -= 1;
        if i%2 == 0 {
            print!("{} ", i);
        }
    }
                                                                                                        //
                                                                                                        print!("\n\n");
                                                                                                        //
    //2. a
    let (imie, wzrost, waga) = ("Jan", 180, 70.5);
    print!("{}, {}, {}", imie, wzrost, waga);
                                                                                                        //
                                                                                                        print!("\n\n");
                                                                                                        //
    //3.
                                                                                                        //
                                                                                                        print!("\n\n");
                                                                                                        //
    let ones = [1; 100];
    print!("{:?}", ones);
    //4.
                                                                                                        //
                                                                                                        print!("\n\n");
                                                                                                        //
    let box_of_ones: [[i32; 100]; 100] = [[1; 100]; 100 ];
    //print!("{:?}", box_of_ones);
    //
    //5. a)
    //
    let (a,b) = (2, 8);
    let c;
    if a > b{
        c = a;
    }
    else { c=b; }
    print!("{}", c);
                                                                                                        //
                                                                                                        print!("\n\n");
                                                                                                        //
    // 5. b)
    let c = if a>b{a} else {b};
    print!("{}", c);
                                                                                                        //
                                                                                                        print!("\n\n");
                                                                                                        //
    // 6.
    let c = 4;
    let biggest = if a>b && a>c {a} else if b>a && b>c {b} else {c};
    print!("{}", biggest);
                                                                                                        //
                                                                                                        print!("\n\n");
                                                                                                        //
    // 7.
    let tab:Vec<i32> = vec![3,5,-3,-2,4,-8,45,63632,-6543];
    for i in tab{
        if i<0{
            print!("{} ", i);
        }
    }
                                                                                                        //
                                                                                                        print!("\n\n");
                                                                                                        //
    // 8. -> funkcje na dole
    let d = dni_miesiaca(2);
    print!("{}", d);
                                                                                                        //
                                                                                                        print!("\n\n");
                                                                                                        //
    // 9. -> funkcje na dole
    let vecek = vec![2,5,6,-2,-4,3,-5,32,-53,32,-3,-78,421,-420, 2];
    let (dod, uje) = policz(&vecek);
    print!("Dodatnie: {} i ujemne: {}", dod, uje);
                                                                                                        //
                                                                                                        print!("\n\n");
                                                                                                        //
    // 10. -> funkcje na dole
    let c = vec2D(4.0, 2.0) + vec2D(-1.0, 9.0);
    print!("wektor: [{},{}]", c.0, c.1);
                                                                                                        //
                                                                                                        print!("\n\n");
                                                                                                        //
    // 11. -> funkcje na dole
    let trj = Trójkąt {a: 3.0, b: 4.0, c: 6.0};
    let o_fn = obwod(&trj);
    let o_met = trj.obwód_m();
    let o_tra = trj.obwód_t();
    print!("Obwód z funkcji = {} , Obwód z metody = {} , Obwód z traitu = {}", o_fn, o_met, o_tra);
                                                                                                        //
                                                                                                        print!("\n\n");
                                                                                                        //
    // 12. -> funkcje na dole
    
    println!("To trójkąt: {}",czy_to_trojkat(&trj)); 
    println!("To trójkąt: {}", czy_to_trojkat(&Trójkąt { a: (4.0), b: (2.0), c: (1.0) }));
    

                                                                                                        //
                                                                                                        print!("\n\n");
                                                                                                        //
    // 13. -> funkcje na dole
    let trojmian = Trójmian::Ogólna { a: (2.0), b: (1.0), c: (1.0) };
    println!("Wartośc trójmianu w punkcie 2.0: {}", trojmian.eval(2.0));

    print!("\n\n");
    let kanoniczna = Trójmian::Kanoniczna { a: 1.0, p: 1.0, q: 0.0 };
    let ogólna_z_kanonicznej = kanoniczna.na_ogólną();
    println!("Wartośc trójmianu w punkcie 4.0: {}", kanoniczna.eval(4.0));
    println!("Wartośc trójmianu w punkcie 4.0: {}", ogólna_z_kanonicznej.eval(4.0));
    if let Trójmian::Ogólna { a, b, c } = ogólna_z_kanonicznej {
        println!("Postać ogólna: {}x² + {}x + {}", a, b, c); // 1x² - 2x + 1
    }
}

// 8.
fn dni_miesiaca(m:u8) -> u8{
    match m{
        1 | 3 | 5 | 7 | 8| 10 | 12 => 31,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => panic!("zły nr miesiąca"),
    }
}

// 9.
fn policz(tab: &Vec<i32>) -> (i32,i32){
    let mut d = 0;
    let mut u = 0;

    for &i in tab{
        //print!("{}, ", i);    

        if i < 0{
            u += 1;
        }
        if i > 0{
            d += 1;
        }
    }
    (d,u)
}

// 10.
#[warn(non_camel_case_types)]
struct vec2D(f64, f64);

impl Add for vec2D{
    type Output = Self;
    fn add(self, rhs: Self) -> Self{
        vec2D(self.0 + rhs.0, self.1 + rhs.1)
    }
}

// 11.

struct Trójkąt {
    a: f64,
    b: f64,
    c: f64,
}

//a
fn obwod(trj:&Trójkąt) -> f64{
    trj.a + trj.b + trj.c  
}

//b
impl Trójkąt{
    fn obwód_m(&self) -> f64{
        &self.a + &self.b + &self.c
    }
}

//c
trait Obwód{
    fn obwód_t(&self) -> f64;
}

impl Obwód for Trójkąt{
    fn obwód_t(&self) -> f64 {
        &self.a + &self.b + &self.c
    }
}


// 12.
fn czy_to_trojkat(t: &Trójkąt) -> bool{

    if t.a+t.b>t.c && t.a+t.c>t.b && t.b+t.c>t.a {true} else {false}
}


// 13.
enum Trójmian{
    Ogólna{a:f64,b:f64,c:f64},
    Kanoniczna{a:f64,p:f64,q:f64},
    Iloczynowa{a:f64,x1:f64,x2:f64},
}

// a)

trait Eval {
    fn eval(&self, x:f64) -> f64;
}

impl Eval for Trójmian{
    fn eval(&self, x:f64) -> f64 {
        match self {
            Trójmian::Ogólna { a, b, c } => a * x.powi(2) + b * x + c,
            Trójmian::Iloczynowa { a, x1, x2 } => a * (x - x1)* (x - x2),
            Trójmian::Kanoniczna { a, p, q } => a * (x - p).powi(2) + q,
        }
    }
}

// b)
impl Trójmian{
    fn na_ogólną(&self) -> Trójmian{
        match self {
            Trójmian::Ogólna { a, b, c } => Trójmian::Ogólna { a: *a, b: *a, c: *a },
            Trójmian::Iloczynowa { a, x1, x2 } => {
                let b = -a * (x1 + x2);
                let c = a * x1 * x2;
                Trójmian::Ogólna { a: (*a), b: (b), c: (c) }
            }
            Trójmian::Kanoniczna { a, p, q } => {
                let b = -2.0 * a * p;
                let c = a * p.powi(2) + q;
                Trójmian::Ogólna { a: (*a), b: (b), c: (c) }
            }
        }
    }
}