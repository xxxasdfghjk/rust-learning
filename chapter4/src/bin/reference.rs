fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    // s1を再び参照可能
    println!("The length of '{}' is {}.", s1, len);
}

// &をつけることにより所有権をもらわない
// 借用
fn calculate_length(s: &String) -> usize {
    // 変更を加えることはできない
    // s.push_str("aa");
    s.len()
}
