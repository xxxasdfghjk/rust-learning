fn main() {
    let s1 = String::from("Hello");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}. ", s2, len);

    // やはりs1はmoveしているため参照不可
    // println!("{}", s1);
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
