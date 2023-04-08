fn main() {
    (0..100)
        .map(|i| {
            if i < 2 {
                false
            } else {
                !(2..i).map(|j| i % j == 0).any(|x| x)
            }
        })
        .enumerate()
        .filter(|&(_, x)| x)
        .for_each(|(i, _)| {
            println!("{} is prime.", i);
        })
}
