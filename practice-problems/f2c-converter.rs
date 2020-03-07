use std::io;

fn main() {
    println!("Enter value in Fahrenheit:");
    let mut f = String::new();
    
    io::stdin().read_line(&mut f)
        .expect("Failed to read line.");

    let f: f64 = f.trim().parse()
        .expect("Please type a number");

    let result: f64 = (f-32.0)*0.55555555555;
    println!("{} Fahrenheit = {} Celcius.", f, result)
}
