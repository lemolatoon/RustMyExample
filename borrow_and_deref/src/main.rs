fn main() {
    let a = vec![1, 2, 3];
    let borrowed_a = &a;
    let b = vec![1, 2, 3];
    println!("equality: {}", *borrowed_a == b);
    println!("a: {:?}, b: {:?}", a, b);

    let mut moved_a = a; // move
    let muttably_borrowed_a = &mut moved_a;
    *muttably_borrowed_a = vec![1, 2, 3, 4];
    println!("moved_a: {:?}", moved_a);
}
