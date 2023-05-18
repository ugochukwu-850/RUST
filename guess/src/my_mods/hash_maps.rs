pub enum allTypes {
    int(i64),
    string(String),
    boolean(bool),
    float(f64),
    HashMap(HashMap<allTypes, allTypes>),
}

use std::{collections::HashMap, hash::Hash};

pub fn hash_maps_creating_hashes() -> allTypes {
    //created a hashmap or dcitionary scores
    let mut scores = HashMap::new();
    //inserted tonew key and values pairs
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    //get an option variable use coied to copit a unwrapto handle a none case
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{}", score);

    //loop trough the the dict hash by taking the ref to those data so asnot to own the hash
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    allTypes::boolean(true)
}

pub fn brut_hash_update() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    //check if there is a yellow string entry
    scores.entry(String::from("Yellow")).or_insert(50);
    //else insert 50
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
}

pub fn updating_hash_table() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).and_modify(|e| {*e+=100}).or_insert(0);
        *count += 1; //here we are increasing the value in the map
                     /*count is a pointer to the value and that is what or_interest returns
                     Test by changing the value of count ++
                     */
    }

    println!("{:?}", map);
}

