fn main() {
    // copy traitをimplementしている要素がある場合moveが発生する
    let tup1 = (1, 2, 3, String::from("aaa"));
    let tup2 = tup1;

    // すべてCopy traitをimplementしている場合moveが起きない
    let tup_ok_1 = (1, 2, 3, "aaa");
    let tup_ok_2 = tup_ok_1;
    println!("{} {}", tup_ok_1.0, tup_ok_2.0);
    println!("{} {}", tup1.0, tup2.0);
}
