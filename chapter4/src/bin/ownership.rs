fn main() {
    let s = String::from("Hello");

    takes_ownership(s);

    // takes_ownershipの引数にmoveされここでもうsは有効ではない
    println!("{}", s);

    // copy traitがimplementsされていたら特に特別なことは起きない
    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_interger: i32) {
    println!("{}", some_interger);
}
