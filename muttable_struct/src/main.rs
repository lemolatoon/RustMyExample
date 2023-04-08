struct Hoge {
    color: String,
}

impl Hoge {
    fn new(color: String) -> Hoge {
        Hoge { color }
    }

    fn red(&mut self) {
        self.color = "red".to_string();
    }

    fn green(&mut self) {
        self.color = "green".to_string();
    }

    fn blue(&mut self) {
        self.color = "blue".to_string();
    }
}

fn main() {
    let mut hoge = Hoge::new("black".to_string());
    hoge.red();
    hoge.green();
}
