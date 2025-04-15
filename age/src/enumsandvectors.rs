pub fn enums_vectors() {
    // ----- ENUMS -----
    // Enums allow for the definition of custom data types

    // Create an Enum for days of week
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    // Define function for Day enum
    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }
    }

    // Use enum to store todays day
    let today: Day = Day::Monday;

    // Perform different actions based on day
    match today {
        Day::Monday => println!("Everyone hates Monday"),
        Day::Tuesday => println!("Donut day"),
        Day::Wednesday => println!("Hump day"),
        Day::Thursday => println!("Pay day"),
        Day::Friday => println!("Almost Weekend"),
        Day::Saturday => println!("Weekend!!!"),
        Day::Sunday => println!("Weekend!!!"),
    }

    // Check if today is a weekend
    println!("Is today the weekend {}", today.is_weekend());

    // ----- VECTORS -----
    // Vectors are like arrays that can grow if mutable
    // They only store values of the same type

    // Create an empty vector with i32
    let _vec1: Vec<i32> = Vec::new();

    // Create a vector with defined values
    let mut vec2 = vec![1, 2, 3, 4];

    // Add values to the end of a vector
    vec2.push(5);

    // Get value by index
    println!("1st : {}", vec2[0]);

    // Verify value exists
    let _second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd : {}", second),
        None => println!("No 2nd value"),
    };

    // Cycle and change values
    for i in &mut vec2 {
        *i *= 2;
    }

    // Cycle through vector values
    for i in &vec2 {
        println!("{}", i);
    }

    // Get number of values in a vector
    println!("Vec Length : {}", vec2.len());

    // Remove and return the last value
    println!("Pop {:?}", vec2.pop());
}
