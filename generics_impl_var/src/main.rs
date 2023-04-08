use std::ops::Add;

struct Point<T, U> {
    x: T,
    y: U,
}

// String: Add<Output =>
// impl Add<>

impl<T> Point<T, i32>
where
    T: Add<Output = T> + Copy,
{
    fn print(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2.0 };
    let p2 = Point { x: 1, y: 2 };
    p2.print(); // ok
    let p3 = Point { x: 1.0, y: 2 };
    p3.print(); // ok
                // p1.sum();
                // p2.sum();
    println!("Hello, world!");
}
