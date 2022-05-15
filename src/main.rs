fn main() {
    use std::io::{stdin,stdout,Write};
    loop {
        let mut input = String::new();
        println!("Enter game size :");
        let size = std::io::stdin().read_line(&mut input).unwrap();
        let is_num = input.trim().parse::<i8>().is_ok();
        println!("Is a number: {}", is_num)
    }


}