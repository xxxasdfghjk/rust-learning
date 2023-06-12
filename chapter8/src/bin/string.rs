fn main() {
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let hello = String::from("こんにちは");
    let hello = String::from("Здравствуйте");
    let mut s = String::from("foo");
    let mut s2 = String::from("bar");
    s.push_str(s2.as_str());
    s2.push_str("add");
    println!("{}", s);

    let s1 = String::from("Hello ");
    let s2 = String::from("world!");
    let s3 = format!("{}-{}", s1, s2);
    println!("{}", s1);
}
