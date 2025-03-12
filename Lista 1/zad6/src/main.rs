fn main() {
    let mut t = [23,53,64,213,32,43,99,43,23,12,43,54,65,76,87,98,9,8,7,6,5,4,3,2,1,0];
    
    //a)
    let sum_t = sum(&t);
    println!("Suma wartości w tablicy t: {}", sum_t);
    //b)
    let asc_t = max_ascend(&t);
    println!("Najdłuższy ciąg rosnący w tablicy t: {:?}", asc_t);
    //c
    accumulate(&mut t);
    println!("Tablica t po modyfikacji akumulacji: {:?}", t);
}

//a)
fn sum(t: &[i32]) -> i32 {
    let mut sum = 0;
    for i in t{
        //println!("{}", i);
        sum += i;
    }
    sum
}

//b)
fn max_ascend(t: &[i32]) -> &[i32] {
    if t.is_empty() {
        return &[]; // Jeśli pusta, zwróć pusty
    }

    let mut max_start = 0; // Początek maxa
    let mut max_end = 0;   // Koniec maxa
    let mut current_start = 0; // Początek teraz

    for i in 1..t.len() {
        if t[i] <= t[i - 1] {
            if (i - current_start) > (max_end - max_start) {
                max_start = current_start;
                max_end = i - 1;
            }
            //println!("{} {}",max_start,max_end);
            current_start = i; // nowy
            //println!("{}",current_start);
        }
    }

    if (t.len() - current_start) > (max_end - max_start + 1) {
        max_start = current_start;
        max_end = t.len() - 1;
    }

    &t[max_start..=max_end] 
}

//c)
fn accumulate(t: &mut [i32]){
    for i in  0..t.len(){
        if i == 1{
            t[i] += t[i-1];
        }
        if i > 1{
            for j in 0..i{
                t[i] += t[j];
            }
        }
        
        //println!("{}", t[i]);
    }
    
}
