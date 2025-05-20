// TODO: Fix the compiler error.
fn main() {
    // this is the initial/original value
    let x = 3;
    println!("Number {x}");

    // this is shadowing, and the first value will be drop once its out of block
    let x = 5; // Don't change this line
    println!("Number {x}");
}
