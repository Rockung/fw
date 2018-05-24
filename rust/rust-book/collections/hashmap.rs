use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    if let Some(score) = scores.get(&team_name) {
        println!("{:?}", score);
    }

    // overwrite a value
    scores.insert(String::from("Blue"), 25);

    // only insert a value if the key has no value
    scores.entry(String::from("Red")).or_insert(200);
    scores.entry(String::from("Yellow")).or_insert(100);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];
    // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // }
}
