pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        println!("hello world!")
    }

    #[test]
    fn stack_demo() {

        let mut stack = Vec::new();
        stack.push(1);
        stack.push(3);
        stack.push(2);
        stack.push(5);
        stack.push(4);

        let top = stack.last().unwrap();
        println!("{}", top);
        // let pop = stack.pop().unwrap();
        // println!("{}", pop);
        let size = stack.len();
        println!("{}", size);
        let is_empty = stack.is_empty();
        println!("{}", is_empty);


    }

    #[test]
    fn test01() {
        let stack: Vec<i32>= Vec::new();
        println!("{:?}", stack);
    }

    #[test]

    fn test02() {
        println!("{}", add(12, 1));
    }
    
}
