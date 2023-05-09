fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn main() {
    let mut str = String::new();
    str.push_str("jfiea f iwafiw jwfj iwa");
    let res = first_word(&str);
    println!("{}", res);
}
