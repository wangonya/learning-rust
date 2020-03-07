use std::io;

fn main() {
    println!("Enter number to find fib:");

    let mut n = String::new();

    io::stdin().read_line(&mut n)
        .expect("Failed to read input");

    let n: u32 = n.trim().parse()
        .expect("Please enter a number");

    let result = fib(n);
    println!("Fib {} = {}", n, result)
}

fn fib(n:u32) -> u32 {
    if n < 3 {
        return 1;
    } else {
        return fib(n-1)+fib(n-2);
    }
}
