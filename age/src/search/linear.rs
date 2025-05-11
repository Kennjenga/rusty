fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &item) in arr.iter().enumerate() {
        if item == target {
            return Some(index);
        }
    }
    None
}

pub fn main() {
    let arr = [1, 2, 3, 4, 5];
    let target = 3;

    match linear_search(&arr, target) {
        Some(index) => println!("Found {} at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}
