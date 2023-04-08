// fn get_add_n_closure(n: usize) -> impl Fn(usize) -> usize {
fn get_add_n_closure(n: usize) -> Box<dyn Fn(usize) -> usize> {
    if n % 2 == 0 {
        Box::new(move |x| x + n)
    } else {
        Box::new(move |x| x + n * 2)
    }
}

fn main() {
    let f2 = get_add_n_closure(2);
    let f5 = get_add_n_closure(5);
    println!("f2(3) = {}", f2(3));
    println!("f5(3) = {}", f5(3));
    println!("Hello, world!");
}
