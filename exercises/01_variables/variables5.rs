fn main() {
    // basically, shadowing let you change the type
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // this change the type to int from string
    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    let number = 3;
    println!("Number plus two is: {}", number + 2);
}
