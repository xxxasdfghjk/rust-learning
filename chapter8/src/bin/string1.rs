fn main() {
    let s1 = String::from("hello");
    let emoji = "😆⭕️";
    for i in emoji.chars() {
        println!("{}", i);
    }
    for i in emoji.bytes() {
        println!("{}", i);
    }
}
