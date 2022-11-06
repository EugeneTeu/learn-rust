fn main() {
    println!("Hello, world!");

    let x: String = String::from("Hello");
    let y = x;
    // x is no longer valid here

    let deep = y.clone();
    // copy heap data here

    // copy trait
    let n1 = 5;
    let n2 = n1;
    // both valid here as integer on stack only data, can copy
}
