use std::io::stdin;

fn main() {
    let mut buffer = String::new();
    let stdin = stdin();
    stdin.read_line(&mut buffer).unwrap();
    let a: f64 = buffer.trim().parse().unwrap();
    buffer.clear();

    stdin.read_line(&mut buffer).unwrap();
    let b: f64 = buffer.trim().parse().unwrap();

    println!("{}", (a / (a - a / b)).round());
}
