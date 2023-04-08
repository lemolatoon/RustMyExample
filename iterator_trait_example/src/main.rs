#[derive(Debug, Clone)]
struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

trait MyIterator<T> {
    fn next(&mut self) -> Option<T>;
}

impl MyIterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        if self.count > 10 {
            return None;
        }

        let count = self.count;
        self.count += 1;
        Some(count)
    }
}

impl MyIterator<i32> for Counter {
    fn next(&mut self) -> Option<i32> {
        if self.count > 10 {
            return None;
        }

        let count = self.count;
        self.count += 1;
        Some(-(count as i32))
    }
}

// impl Iterator for Counter {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.count > 10 {
//             return None;
//         }

//         let count = self.count;
//         self.count += 1;
//         Some(count)
//     }
// }

fn main() {
    println!("Hello, world!");
    let mut counter = Counter::new();
    assert_eq!(Some(0), <Counter as MyIterator<u32>>::next(&mut counter));
    assert_eq!(Some(1), <Counter as MyIterator<u32>>::next(&mut counter));
    assert_eq!(Some(-2), <Counter as MyIterator<i32>>::next(&mut counter));
}
