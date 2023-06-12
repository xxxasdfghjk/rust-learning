fn main() {
    enum SpreadseetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadseetCell::Int(3),
        SpreadseetCell::Text(String::from("blue")),
        SpreadseetCell::Float(10.12),
    ];
    let sample1 = SpreadseetCell::Int(10);
    if let SpreadseetCell::Int(some) = sample1 {
        println!("{}", some)
    };
}
