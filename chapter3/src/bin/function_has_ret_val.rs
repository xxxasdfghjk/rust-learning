fn five() -> i32 {
    return 5;
}

fn plus_one(x: i32) -> i32 {
    // セミコロン不要
    // 最後の式が式として評価される
    x + 1
}

fn main() {
    let x = plus_one(3);
    println!("The value of x is; {}", x)
}
