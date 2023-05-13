fn main() {
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    let Some(4) = some_u8_value else {
        println!("none");
        return;
    };
}
