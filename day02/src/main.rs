

fn main() {

    let n = 17;
    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0{
        println!("{} is postive", n);
    } else {
        println!("{} is zero", n);
    }

    let big_n = 
        if n < 10 && n > -10 {
            10 * n
        } else {
            println!(", and is a big number, halve the number");
            n / 2
        };

    println!("{} -> {}", n, big_n);

    let result = add(1, 6);
    println!("{}", result);


}


fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}