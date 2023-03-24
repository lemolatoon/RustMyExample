fn main() {
    {
        let s = "hello";
        println!("{}", s);
    }
    // println!("{}", s);
    //

    {
        let mut s = String::from("hello");

        s.push_str(", world!"); // push_str()関数は、リテラルをStringに付け加える

        println!("{}", s); // これは`hello, world!`と出力する
    }
    // OS にメモリを返している
}
