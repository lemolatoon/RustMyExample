pub mod second;
pub mod third;

use second::hello;
use third::return_three::return_three;

fn main() {
    println!("Hello, world!");
    hello();
    println!("returned: {}", return_three());
}
