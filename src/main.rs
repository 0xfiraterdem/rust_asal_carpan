use std::io;
use std::io::Write;

fn prime_factor_func(mut sayi: i32) -> Vec<i32> {
    let mut prime_factor = Vec::new();
    match sayi {
        2 => prime_factor.push(2),
        _ => for i in 2..sayi {
            if sayi % i == 0 {
                prime_factor.push(i);
                while sayi % i == 0 { sayi /= i }
            }
        }
    }
    prime_factor
}

fn main() {
    print!(">>>");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input).unwrap();
    let num: i32 = input.trim().parse().expect("Value is not a integer...");
    println!("{} sayısının asal çarpanları: {:?}",num,prime_factor_func(num));
}
