fn main() {
    // u8型の配列の例
    let arr1: [u8; 3] = [1, 2, 3];
    for (i, &item) in arr1.iter().enumerate() {
        println!("Index {}: Value {}", i, item);
    }

    // i32型の配列の例
    let arr2: [i32; 3] = [4, 5, 6];
    for (i, &item) in arr2.iter().enumerate() {
        println!("Index {}: Value {}", i, item);
    }
}
