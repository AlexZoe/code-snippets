use std::collections::HashMap;

fn main() {
    // create new hashmap
    let mut scores = HashMap::new();

    // insert string key and i32 value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let mut teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // combine two vectors into hashmap which will invalidate vectors
    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // teams has been moved and can no longer be used
//    teams.push(String::from("hi"));

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // field_name and field_value get moved into hashmap and are no longer valid afterwards
    map.insert(field_name, field_value);

    AccessMap();
    UpdateBasedOnValue();
}


fn AccessMap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // access by key
    let score = scores.get(&team_name);

    // iterate key/value pair
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // overwrite value associated with key
    scores.insert(String::from("Blue"), 30);

    // only insert if key does not exist
    scores.entry(String::from("Yellow")).or_insert(50);
    // does not get inserted as key already exist
    scores.entry(String::from("Blue")).or_insert(50);
}


fn UpdateBasedOnValue() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
