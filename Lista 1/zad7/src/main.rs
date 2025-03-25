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
    
    let a_b = [rectangle1.len_a, rectangle1.len_b];
    println!("\nObwód prostokąta wynosi: {}", obwod(a_b[0], a_b[1]));

    println!("Pole prostokąta wynosi: {}", pole(a_b[0], a_b[1]));
    
    
    println!("\nObecna pozycja prostokąta: {:?}", rectangle1.pos_c);
    let mut poz = -2;
    let mut pion = 3;
    przesun_o_wektor(&mut rectangle1.pos_c, [poz,pion]);
    println!("Przesuńmy prostokąt o {} w osi x oraz o {} w osi y\nNowa pozycja prostokątu: {:?}", poz, pion, rectangle1.pos_c);
    
    poz = 4;
    pion = 90;
    println!("\nPrzesuńmy prostokąt do punktu o współrzędnych [{poz}, {pion}]");
    przesun_do_punktu(&mut rectangle1.pos_c, [poz,pion]);
    println!("Nowa pozycja prostokątu: {:?}", rectangle1.pos_c);

    println!("\nBoki przed obrotem: a = {}, b = {}", rectangle1.len_a, rectangle1.len_b);
    obroc_o_90(&mut rectangle1.len_a, &mut rectangle1.len_b);
    println!("Obrót o 90 stopni: a = {}, b = {}", rectangle1.len_a, rectangle1.len_b);
    
    skaluj(&mut rectangle1.len_a, &mut rectangle1.len_b, 4);
    
}


fn obwod(a: i32, b: i32) -> i32{
    let ob = 2*a.abs() + 2*b.abs();
    ob
}

fn pole(a: i32, b: i32) -> i32{
    let pol = a.abs()*b.abs();
    pol
}

fn przesun_o_wektor(cpos: &mut [i32;2], moving: [i32;2]){
    cpos[0] += moving[0];
    cpos[1] += moving[1];
}

fn przesun_do_punktu(cpos: &mut [i32;2], moving: [i32;2]){
    cpos[0] = moving[0];
    cpos[1] = moving[1];
}

fn obroc_o_90(a: &mut i32, b: &mut i32){
    let temp= *a;
    //println!("\n{}\n{}", a, b);
    *a = *b;
    *b = temp;
    //*b = *a;
    //println!("\n{}\n{}", a, b);
    //a = temp;
}

fn skaluj(a: &mut i32, b: &mut i32, scale: i32){
    *a *= scale;
    *b *= scale;
    //print!("\n\n{}, {}", a,b);
}


#[test]
fn test_obwod(){
    assert_eq!(obwod(2, 4), 12);
    assert_eq!(obwod(10,10), 40);
    assert_eq!(obwod(5, 3), 16);
}

#[test]
fn test_pole(){
    assert_eq!(pole(2, 4), 8);
    assert_eq!(pole(10,10), 100);
    assert_eq!(pole(5, 3), 15);
}

#[test]
fn test_przesun_o_wektor(){
    let mut pos = [0, 0];
    przesun_o_wektor(&mut pos, [2, 3]);
    assert_eq!(pos, [2, 3]);
    przesun_o_wektor(&mut pos, [1, 1]);
    assert_eq!(pos, [3, 4]);
}

#[test]
fn test_przesun_do_punktu(){
    let mut pos = [0, 0];
    przesun_do_punktu(&mut pos, [2, 3]);
    assert_eq!(pos, [2, 3]);
    przesun_do_punktu(&mut pos, [1, 1]);
    assert_eq!(pos, [1, 1]);
}

#[test]
fn test_obroc_o_90(){
    let mut a = 2;
    let mut b = 4;
    obroc_o_90(&mut a, &mut b);
    assert_eq!(a, 4);
    assert_eq!(b, 2);
    obroc_o_90(&mut a, &mut b);
    assert_eq!(a, 2);
    assert_eq!(b, 4);
}

#[test]
fn test_skaluj(){
    let mut a = 2;
    let mut b = 4;
    skaluj(&mut a, &mut b, 3);
    assert_eq!(a, 6);
    assert_eq!(b, 12);
    skaluj(&mut a, &mut b, 2);
    assert_eq!(a, 12);
    assert_eq!(b, 24);
}