fn big_latain(st: &str) -> String {
    if st.len() == 0 {
        return String::from("");
    }
    if {
        match st.chars().nth(0) {
            Some(elem) => "aiueo".chars().any(|c| -> bool { c == elem }),
            None => false,
        }
    } {
        format!("{}-{}", &st, "hay").to_string()
    } else {
        format!("{}-{}{}", &st[1..], &st[0..1], "ay").to_string()
    }
}

fn main() {
    println!("{}", big_latain("apple"));
    println!("{}", big_latain("first"));
    println!("{}", big_latain("google"));
    println!("{}", big_latain(""));
}
