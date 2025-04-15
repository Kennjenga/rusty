pub fn arrays() {
    // ----- ARRAYS -----
    // Elements in an array must be of the same type
    // and have a fixed size
    let arr_1 = [1, 2, 3, 4];

    // Get value by index starting at 0
    println!("1st : {}", arr_1[0]);

    // Get array length
    println!("Length : {}", arr_1.len());

    // ----- LOOP -----
    // Create an infinite loop that ends when break is called
    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_idx = 0;
    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue; // Goes to beginning of loop
        }

        if arr_2[loop_idx] == 9 {
            break; // Breaks out of loop
        }

        println!("Val : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    // ----- WHILE LOOP -----
    // Looping based on a condition
    loop_idx = 0;
    while loop_idx < arr_2.len() {
        println!("Arr : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    // ----- FOR LOOP -----
    // For works better for cycling through collections
    for val in arr_2.iter() {
        println!("Val : {}", val);
    }
    // Start Here
    // ----- TUPLES -----
    // Tuples can contain multiple data types in a list of fixed size
    // We convert to strings with to_string()
    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);

    // You can get values by index starting at 0
    println!("Name : {}", my_tuple.1);

    // You can assign values to multiple variables
    let (v1, _v2, _v3) = my_tuple;
    println!("Age : {}", v1);
}
