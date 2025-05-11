fn getsum(x: i32, y: i32) -> i32 {
    x + y
}
fn get2(x: i32) -> (i32, i32) {
    (x + 1, x + 2)
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &num in list.iter() {
        sum += num;
    }
    sum
}

pub fn main() {
    println!("Sum: {}", getsum(10, 27));
    println!("the 2 vals are: {:?}", get2(142));
    // Create a mutable vector
    let mut vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    // Add a value to the vector
    vec.push(6);
    // sum the vector
    println!("Sum of vector: {}", sum_list(&vec));
    let list: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Sum of list: {}", sum_list(&list));
}
