use std::collections::HashMap;

fn main() {
    //HashMap作成
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 値の取り出し
    if let Some(num) = scores.get("Red") {
        println!("{}", num);
    } else {
        println!("None!")
    }

    // entry
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    // word_count
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    for word in &map {
        println!("{};{}", word.0, word.1);
    }

    // iterate
    for (key, value) in &scores {
        println!("{}:{}", key, value)
    }

    // collectから型指定でHashMap生成
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // field_nameの所有権はmapに移動する
    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");
    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);
    // println!("{}", filed_name);
}
