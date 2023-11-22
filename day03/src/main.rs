










fn main() {
    

    let mut n = 5;
    while n > 0 {
        n = n + 1;
        if n > 10 {
            println!("{}", n);
            break;
        }
    }

    let mut m = 10;
    loop {
        m += 1;
        println!("{}", m);
        if m > 100 {
            break;
        }
    }
}