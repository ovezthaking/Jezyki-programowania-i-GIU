struct Rectangle {
    pos_c: [i32; 2],
    len_a: i32,
    len_b: i32,
}

fn main() {
    let mut rectangle1 = Rectangle{
        pos_c: [0, 0],
        len_a: 2,
        len_b: 4,
    };
    
    //let a_b = [rectangle1.len_a, rectangle1.len_b];
    println!("\nObwód prostokąta wynosi: {}", rectangle1.obwod());

    println!("Pole prostokąta wynosi: {}", rectangle1.pole());
    
    
    println!("\nObecna pozycja prostokąta: {:?}", rectangle1.pos_c);
    let mut poz = -2;
    let mut pion = 3;
    rectangle1.przesun_o_wektor([poz, pion]);
    println!("Przesuńmy prostokąt o {} w osi x oraz o {} w osi y\nNowa pozycja prostokątu: {:?}", poz, pion, rectangle1.pos_c);
    
    poz = 4;
    pion = 90;
    println!("\nPrzesuńmy prostokąt do punktu o współrzędnych [{poz}, {pion}]");
    rectangle1.przesun_do_punktu([poz, pion]);
    println!("Nowa pozycja prostokątu: {:?}", rectangle1.pos_c);

    println!("\nBoki przed obrotem: a = {}, b = {}", rectangle1.len_a, rectangle1.len_b);
    rectangle1.obroc_o_90();
    println!("Obrót o 90 stopni: a = {}, b = {}", rectangle1.len_a, rectangle1.len_b);
    
    rectangle1.skaluj(4);
    
}

impl Rectangle{
    
    fn obwod(&self) -> i32{
        let ob = 2*self.len_a.abs() + 2*self.len_b.abs();
        ob
    }

    fn pole(&self) -> i32{
        let pol = self.len_a.abs()*self.len_b.abs();
        pol
    }

    fn przesun_o_wektor(&mut self, moving: [i32; 2]) {
        self.pos_c[0] += moving[0];
        self.pos_c[1] += moving[1];
    }

    fn przesun_do_punktu(&mut self, moving: [i32; 2]) {
        self.pos_c[0] = moving[0];
        self.pos_c[1] = moving[1];
    }

    fn obroc_o_90(&mut self) {
        let temp = self.len_a;
        self.len_a = self.len_b;
        self.len_b = temp;
    }

    fn skaluj(&mut self, scale: i32) {
        self.len_a *= scale;
        self.len_b *= scale;
    }

}
#[test]
fn test_obwod(){
    let rect = Rectangle { pos_c: [0, 0], len_a: 2, len_b: 4 };
    assert_eq!(rect.obwod(), 12);
}

#[test]
fn test_pole(){
    let rect = Rectangle { pos_c: [0, 0], len_a: 5, len_b: 3 };
    assert_eq!(rect.pole(), 15);
}

#[test]
fn test_przesun_o_wektor(){
    let mut rect = Rectangle { pos_c: [0, 0], len_a: 0, len_b: 0 };
    rect.przesun_o_wektor([2, 3]);
    assert_eq!(rect.pos_c, [2, 3]);
    rect.przesun_o_wektor([1, 1]);
    assert_eq!(rect.pos_c, [3, 4]);
}

#[test]
fn test_przesun_do_punktu() {
    let mut rect = Rectangle { pos_c: [0, 0], len_a: 0, len_b: 0 };
    rect.przesun_do_punktu([2, 3]);
    assert_eq!(rect.pos_c, [2, 3]);
    rect.przesun_do_punktu([1, 1]);
    assert_eq!(rect.pos_c, [1, 1]);
}

#[test]
fn test_obroc_o_90() {
    let mut rect = Rectangle { pos_c: [0, 0], len_a: 2, len_b: 4 };
    rect.obroc_o_90();
    assert_eq!(rect.len_a, 4);
    assert_eq!(rect.len_b, 2);
    rect.obroc_o_90();
    assert_eq!(rect.len_a, 2);
    assert_eq!(rect.len_b, 4);
}

#[test]
fn test_skaluj() {
    let mut rect = Rectangle { pos_c: [0, 0], len_a: 2, len_b: 4 };
    rect.skaluj(3);
    assert_eq!(rect.len_a, 6);
    assert_eq!(rect.len_b, 12);
    rect.skaluj(2);
    assert_eq!(rect.len_a, 12);
    assert_eq!(rect.len_b, 24);
}