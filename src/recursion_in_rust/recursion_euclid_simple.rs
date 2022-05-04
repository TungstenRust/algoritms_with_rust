use std::io;
use std::str::FromStr;
use std::cmp::Ordering;

fn get_number(prompt: &str) -> u32 {
    println!("{}", prompt);

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("no input!");

    u32::from_str(input.trim()).unwrap()
}

fn main() {
    let x = get_number("Enter the first number: ");
    let y = get_number("Enter the second number: ");

    println!("The GCD of {} and {} is {}", x, y, euclid(x, y));
}

fn euclid(m: u32, n: u32) -> u32 {
    assert!(m > 0 && n > 0);

    match m.cmp(&n) {
        Ordering::Equal => m,
        Ordering::Less => euclid(m, n-m),
        Ordering::Greater => euclid(m-n, n),
    }
}