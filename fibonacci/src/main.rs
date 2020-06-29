use std::env;

fn fibonacci(n: &u32) -> u32 {
    if *n == 0 {
        return 0;
    } else if *n == 1 {
        return 1;
    } else {
        return fibonacci(&(n - 1)) + fibonacci(&(n - 2));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = &args[1];
    let n: u32 = n.trim().parse().expect("Please enter a number!");
    let res = fibonacci(&n);
    println!("The {}th fibonacci number is: {}", n, res);
}
