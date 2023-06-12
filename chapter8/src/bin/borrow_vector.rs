fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    // OKな例
    let first = &v[0]; // 不変参照スタート
    println!("The first element is;{}", first); // 不変参照エンド
    v.push(6); //可変参照 スタート　エンド

    // NGな例
    let first = &v[0]; // 不変参照スタート
    v.push(6); //可変参照 スタート　エンド
    println!("The first element is;{}", first); // 不変参照エンド
}
