use std::io;

fn main() {
    println!("Welcome to calculator. Choose your operation below:");

    println!(
        "1. Sum\n2. Subtraction\n3. Multiplication\n4. Division\n5. Exponentiation\n6. Square root\n0. Exit"
    );

    let mut option = String::new();

    io::stdin().read_line(&mut option).unwrap();

    let option: u8 = option
        .trim()
        .parse()
        .expect("Your option needs to be an integer.");

    match option {
        0 => println!("Bye bye!"),
        1..=5 => two_operators(option),
        6 => square_root(),
        _ => println!("Unrecognized option. Try running calc again!"),
    }
}

fn two_operators(option: u8) {
    let mut n1 = String::new();
    let mut n2 = String::new();

    println!("Input the first number:");
    io::stdin().read_line(&mut n1).unwrap();
    println!("Input the second number:");
    io::stdin().read_line(&mut n2).unwrap();

    let n1: f64 = n1.trim().parse().expect("You must input a valid number.");
    let n2: f64 = n2.trim().parse().expect("You must input a valid number.");

    match option {
        1 => println!("{} + {} = {}", n1, n2, n1 + n2),
        2 => println!("{} - {} = {}", n1, n2, n1 - n2),
        3 => println!("{} * {} = {}", n1, n2, n1 * n2),
        4 => println!("{} / {} = {}", n1, n2, n1 / n2),
        5 => println!("{} ^ {} = {}", n1, n2, n1.powf(n2)),
        _ => println!("Unrecognized operation. Try running calc again!"),
    }
}

fn square_root() {
    let mut factor = String::new();

    println!("Input the number to get the square root of:");
    io::stdin().read_line(&mut factor).unwrap();

    let factor: f64 = factor
        .trim()
        .parse()
        .expect("You must input a valid number.");

    println!("sqrt({}) = {}", factor, factor.sqrt());
}
