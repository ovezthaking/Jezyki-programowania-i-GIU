fn main() {
    let nwd = nwd(9321, 234);
    println!("{nwd}");
}

fn nwd(mut a:i32, mut b:i32) -> i32{

    //let r = a%b;
    //println!("{r}");

    let result = loop{
        let r = a%b;
        //println!("{r}");
        a = b;
        b = r;
        

        if r == 0{
            break a;
        }

    };
    result
}

#[test]
fn test_nwd(){
    assert_eq!(nwd(282, 78), 6);
}