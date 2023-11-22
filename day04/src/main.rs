


fn main() {

    let r = for_loop(101);
    println!("{}", r);
    let r = while_loop(100);
    println!("{}", r);
    
}


fn for_loop(n: i32) -> i32 {

    let mut res = 0;
    for i in 1..n {
        res += i;
    }
    res
}

fn while_loop(n: i32) -> i32 {

    let mut res = 0;
    let mut i = 1;
    while i <= n {
        res += i;
        i += 1;
    }
    res
} 