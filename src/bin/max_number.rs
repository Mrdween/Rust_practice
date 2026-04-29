use std::io;
use std::cmp::Ordering;

fn largest_i32<'a>(first: &'a i32, second: &'a i32) -> Result<&'a i32, io::Error> {
    match first.cmp(&second) {
        Ordering::Less => Ok(second),
        Ordering::Greater => Ok(first),
        Ordering::Equal => Err(io::Error::other("числа равны")),
    }
}
fn main() {
    let mut input_1: String = String::new();
    let mut input_2: String = String::new();


    io::stdin()
    .read_line(&mut input_1)
    .expect("Строка не распознана");

    io::stdin()
    .read_line(&mut input_2)
    .expect("Строка не распознана");

    let num1: i32 = input_1.trim().parse().unwrap();
    let num2: i32 = input_2.trim().parse().unwrap();

    let largest_num = largest_i32(&num1, &num2);
    println!("{largest_num:?}");
}
