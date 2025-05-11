// ----- HASH MAPS -----
// Hash maps are used to store key / value pairs
use std::collections::HashMap;

pub fn main() {
    let mut heroes = HashMap::new();
    let batman = String::from("Enga Dev");

    // Insert in hashmap (To overwrite use the same key)
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    // Insert with variable
    heroes.insert("Batman", &batman);

    // Iterate over hashmap
    for (k, v) in heroes.iter() {
        println!("{} = {}", k, v);
    }

    // Length of hashmap
    println!("Length : {}", heroes.len());

    // Check for key in hashmap
    if heroes.contains_key(&"Batman") {
        // Get value with key
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("Batman is a hero, {}", x),
            None => println!("Batman is not a hero"),
        }
    }
}
