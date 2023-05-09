fn main() {
    let mut s = String::from("hello world");
    let s1 = first_word("fjieaj iffi jwaifiwj iajfwj ji");
    s.clear();
    println!("{}", s1);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
