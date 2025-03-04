/*
Napisz program, który wyprodukuje ładnie sformatowaną table wartość funkcji trygo-
nometrycznych w postaci tabeli zawierających kolumny: kąt, sinus, cosinus, tangens.
dla wartości kątów od 0o do 45o .
Wskazówka: println!("| {x:2} | {s:5} | {c:5} | {t:5} |").
*/
use std::f64::consts::PI;



fn main() {
    println!("|-----|---------|---------|---------|");
    println!("| kąt |  sinus  | cosinus | tangens |");
    println!("|-----|---------|---------|---------|");
    for angle in 0..=45 {
        
        let angle = angle as f64;
        let radians = angle as f64* PI / 180.0;
        let sin = radians.sin();
        let cos = radians.cos();
        let tan = radians.tan();
        println!("| {:^3} | {:^7.4} | {:^7.4} | {:^7.4} |", angle, sin, cos, tan);
        println!("|-----|---------|---------|---------|");
    }
    //println!("|-----|---------|---------|---------|");
}