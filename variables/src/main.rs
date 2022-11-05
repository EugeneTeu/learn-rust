fn plus_one(x: i32) -> i32 {
    return x + 1;
}

fn fibon(n: i32) -> i32 {
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return 1;
    }

    return fibon(n - 1) + fibon(n - 2);
}

fn main() {
    let x = 6;
    println!("Value of x is {}", x);
    let result = plus_one(x);
    println!("result {}", result);
    println!("Hello, world!");

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    let val: i32 = fibon(0);
    println!("{}", val);
}
