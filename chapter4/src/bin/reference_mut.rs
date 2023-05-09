fn main() {
    let mut s1 = String::from("hello");
    // 可変参照を渡す
    calculate_length(&mut s1);
    // s1を再び参照可能
    println!("{} {}", s1, get_length(&s1));

    let r1 = &s1; // 問題なし
    let r2 = &s1; // 問題なし
    let r3 = &mut s1; // 大問題！
    println!("{} {} {}", r1, r2, r3);
}

// 可変参照をもらうことにより中身を変更可能
fn calculate_length(some_string: &mut String) {
    some_string.push_str(" postfix");
}
fn get_length(some_string: &String) -> usize {
    some_string.len()
}
