fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("Hello");
    let s3 = takes_and_gives_back(s2);

    // s2の所有権はtakes_and_gives_back関数へmove
    // s3の所有権はtakes_and_gives_backに戻る
    println!("{} {} {} ", s1, s2, s3)
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
