fn main() {
    let mut x = 5;
    const CONSTANT: usize = 100;
    println!("The value of CONSTANT is: {}", CONSTANT);
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5; // 5

    let y = y + 1; // 6

    {
        let z = 5;
        let y = y * 2; // 12
        println!("The value of y in the inner scope is: {}", y);
        println!("The value of z is: {}", z);
    }

    // println!("The value of z is: {}", z);

    println!("The value of y is: {}", y);

    let some_strings = "aaa";
    println!("The value of spaces is: {}", some_strings);

    let some_strings = some_strings.len();
    println!("The value of spaces is: {}", some_strings);
}
