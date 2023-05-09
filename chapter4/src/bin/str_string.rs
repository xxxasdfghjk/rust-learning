fn main() {
    let mut s = String::from("string");
    let mut t = "literal";
    s.push_str(" postfix");
    println!("{}", s);
}
