trait Summary {
    fn summarize_author(&self) -> String;
}

struct Tweet {
    username: String,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
    }
}

fn call_with_type_impls_summary<T: Summary>(item: T) {
    println!("Summary: {}", item.summarize_author());
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
    };
    println!("{}", tweet.summarize_author());
    call_with_type_impls_summary(tweet);
}
