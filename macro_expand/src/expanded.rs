#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn main() {
    let vec = <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([1, 2, 3, 5]));
    let n = 5;
    {
        ::std::io::_print(format_args!("Hello, world! number: {0}\n", n));
    };
    {
        ::std::io::_print(format_args!("Hello, world! vec: {0:?}\n", vec));
    };
}
