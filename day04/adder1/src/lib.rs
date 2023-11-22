pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test01() {
        // let r = for_loop(101);
        let r = while_loop(100);
        println!("{}", r);
    }
}



pub fn for_loop(n: i32) -> i32 {

    let mut res = 0;

    for i in 1..n {
        res += i
    }

    res

}


pub fn while_loop(n: i32) -> i32 {


    let mut res = 0;
    let mut m = 1;

    while m <= n {
        res += m;
        m += 1;

    }

    res

}

