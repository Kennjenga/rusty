pub fn main() {
    // ----- STRINGS -----
    // There are 2 types of strings
    // 1. String : Vector of bytes that can be changed
    // 2. &str : Points to the string and allows for viewing

    // Create an empty growable string
    let mut st1 = String::new();

    // Insert a character at the end of a string
    st1.push('A');

    // Insert a string at the end
    st1.push_str(" word");

    // Iterate through words by splitting at whitespace
    for word in st1.split_whitespace() {
        println!("{}", word);
    }

    // Replace a string (Use "" for deleting)
    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    // Create string of characters
    let st3 = String::from("x r t b h k k a m c");

    // Convert to a vector
    let mut v1: Vec<char> = st3.chars().collect();

    // Sort characters
    v1.sort();

    // Remove duplicates
    v1.dedup();

    // Cycle through vector
    for char in v1 {
        println!("{}", char);
    }

    // Create a string literal
    let st4: &str = "Random string";

    // Convert to heap allocated String
    let mut st5: String = st4.to_string();
    println!("{}", st5);

    // Convert string into an array of bytes
    let _byte_arr1 = st5.as_bytes();

    // Get a slice of a string from index 0 to 5
    let st6 = &st5[0..6];
    println!("{}", st6);

    // Get length of string
    println!("String Length : {}", st6.len());

    // Delete values in a string if mutable
    st5.clear();

    // Combine strings
    let st6 = String::from("Just some");
    let st7 = String::from("words");

    // st6 can no longer be used
    // You can only add a reference to a string to another
    let st8 = st6 + &st7;

    // Cycle through letters in a string and print unicode
    for char in st8.bytes() {
        println!("{}", char);
    }

    // Cycle through letters in a string and print characters
    for char in st8.chars() {
        println!("{}", char);
    }
}
