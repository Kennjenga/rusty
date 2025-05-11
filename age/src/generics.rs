use std::ops::Add;

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

pub fn main() {
    // ----- GENERIC TYPES -----
    // We can specify the data type to be used at a later time with generics
    // It is mainly used when we want to create functions that can work with
    // multiple data types. It is used with structs, enums, traits, etc.
    // which we'll talk about later
    println!("5 + 4 = {}", get_sum_gen(5, 4));
    println!("5.2 + 4.6 = {}", get_sum_gen(57.2, 14.86));
}
