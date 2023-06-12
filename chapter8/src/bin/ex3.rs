use std::collections::HashMap;

fn add_user(text_command: &str, hash_map: &mut HashMap<String, Vec<String>>) -> bool {
    let mut splited_text = text_command.split_whitespace();
    let (Some(command), Some(username), Some(description), Some(group)) = (
        splited_text.nth(0),
        splited_text.nth(0),
        splited_text.nth(0),
        splited_text.nth(0),
) else {
        return false;
    };
    if command == "Add" && description == "to" {
        let entry = hash_map.entry(group.to_string()).or_insert(Vec::new());
        if !entry.contains(&username.to_string()) {
            entry.push(username.to_string());
            entry.sort();
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

fn main() {
    let mut hash_map: HashMap<String, Vec<String>> = HashMap::new();
    println!("{}", add_user("Add Amir to Sales", &mut hash_map));
    println!("{}", add_user("Add Bob to Engineer", &mut hash_map));
    println!("{}", add_user("Add Dick to Engineer", &mut hash_map));
    println!("{}", add_user("Add Emily to Manifacture", &mut hash_map));
    for (key, value) in &hash_map {
        println!("{}:{}", key, value.join(","));
    }
}
