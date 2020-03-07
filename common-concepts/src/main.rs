fn main() {
    // variables
    let mut x = 23;
    println!("x before mut = {}", x);
    x = 32;
    println!("x after mut = {}", x);

    // shadowing
    let spaces = "      ";
    let spaces = spaces.len();
    println!("spaces len = {}", spaces);

    // consts
    const MAX_POINTS: u32 = 100_000;
    println!("const max points = {}", MAX_POINTS);

    // tuples
    let tup = (1, 2, 'c', "str"); // '' for char, "" for str
    println!("tup0 = {}", tup.0);
    println!("tup1 = {}", tup.1);
    println!("tup2 = {}", tup.2);
    println!("tup3 = {}", tup.3);

    // arrays
    let arr = [1,2,3];
    let arrr = ['c'; 5];
    println!("arr0 = {}", arr[0]);
    println!("arrr3 = {}", arrr[3]);

    // calling other fns
    another_fn();
    // with params
    pr_fn(2, false, "tdf");
    // with return value
    let f = rt_5();
    println!("rt from rt_5 = {}", f);

    // if
    if rt_5() == 5 {
        println!("rt_5 is indeed 5.")
    } // you can continue to use else / else if ...
    // used with let
    let fv = if rt_5() == 5 {5} else {3};
    println!("fv = {} set using if stmt", fv);

    // loop with break
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result from loop = {}", result);

    // while loop
    let mut num = 3;
    while num != 0 {
        println!("{}", num);
        num = num - 1;
    }
    println!("while loop LIFTOFF!!!");

    // for loop
    for el in arr.iter() {
        println!("for loop el is {}", el);
    }

    // liftoff code in for loop
    for num in (1..4).rev() {
        println!("{}", num);
    }
    println!("for loop LIFTOFF!!!")
}

fn another_fn () {
    println!("another fn!!");
}

fn pr_fn(x:i32, y:bool, z:&str) {
    println!("received x={}, y={} and z={}", x,y,z);
}

fn rt_5() -> i32 {
    5 // or - return 5;
}
