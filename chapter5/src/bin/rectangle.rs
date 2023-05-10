#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };
    println!(
        "The are of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!("rect1 is {:#?}", rect1);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
