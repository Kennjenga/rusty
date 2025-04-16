use std::cmp::Ordering;

/// Performs a binary search on a sorted slice.
///
/// # Arguments
///
/// * `arr` - A sorted slice (`&[T]`) to search within.
/// * `target` - The value (`T`) to search for.
///
/// # Type Parameters
///
/// * `T` - The type of the elements in the slice. Must implement `Ord`
///         for comparison (`<`, `>`, `==`).
///
/// # Returns
///
/// * `Some(usize)` - The index of the target element if found.
/// * `None` - If the target element is not found in the slice.
///
/// # Precondition
///
/// The input slice `arr` **must** be sorted in ascending order.
/// The behavior is undefined if the slice is not sorted.
fn binary_search<T: Ord>(arr: &[T], target: T) -> Option<usize> {
    // Handle empty slice case early
    if arr.is_empty() {
        return None;
    }

    let mut low: usize = 0;
    // Use arr.len() - 1 for the initial high index
    let mut high: usize = arr.len() - 1;

    // Loop while the search space [low, high] is valid
    // Use <= because low and high could converge on the same index
    while low <= high {
        // Calculate the middle index.
        // Using low + (high - low) / 2 avoids potential overflow
        // compared to (low + high) / 2 for very large indices.
        let mid = low + (high - low) / 2;

        // Compare the middle element with the target
        match arr[mid].cmp(&target) {
            // Middle element is less than the target:
            // Target must be in the right half (if it exists).
            // Discard the left half including the middle element.
            Ordering::Less => {
                low = mid + 1;
            }
            // Middle element is greater than the target:
            // Target must be in the left half (if it exists).
            // Discard the right half including the middle element.
            Ordering::Greater => {
                // Need to handle the case where mid is 0 to prevent underflow
                // when assigning high = mid - 1. However, the usize type
                // wraps around on underflow. The `while low <= high`
                // condition will correctly terminate the loop eventually
                // if high wraps around to a large value.
                // A check `if mid == 0 { return None; }` could be added
                // for early exit if the target is smaller than the first element.
                if mid == 0 {
                    // If mid is 0 and it's greater than target, target can't be in the slice
                    return None;
                }
                high = mid - 1;
            }
            // Middle element is equal to the target: Found it!
            Ordering::Equal => {
                return Some(mid);
            }
        }
    }

    // If the loop finishes without returning, low has become > high,
    // meaning the search space is empty and the target was not found.
    None
}

pub fn main() {
    let sorted_numbers = vec![2, 5, 8, 12, 16, 23, 38, 56, 72, 91];
    let empty_vec: Vec<i32> = vec![];

    // --- Test Case 1: Target found ---
    let target1 = 23;
    println!("Searching for {} in {:?}", target1, sorted_numbers);
    match binary_search(&sorted_numbers, target1) {
        Some(index) => println!("Found {} at index {}", target1, index), // Expected: Found 23 at index 5
        None => println!("{} not found", target1),
    }
    println!("---");

    // --- Test Case 2: Target not found (within range) ---
    let target2 = 40;
    println!("Searching for {} in {:?}", target2, sorted_numbers);
    match binary_search(&sorted_numbers, target2) {
        Some(index) => println!("Found {} at index {}", target2, index),
        None => println!("{} not found", target2), // Expected: 40 not found
    }
    println!("---");

    // --- Test Case 3: Target not found (smaller than first element) ---
    let target3 = 1;
    println!("Searching for {} in {:?}", target3, sorted_numbers);
    match binary_search(&sorted_numbers, target3) {
        Some(index) => println!("Found {} at index {}", target3, index),
        None => println!("{} not found", target3), // Expected: 1 not found
    }
    println!("---");

    // --- Test Case 4: Target not found (larger than last element) ---
    let target4 = 100;
    println!("Searching for {} in {:?}", target4, sorted_numbers);
    match binary_search(&sorted_numbers, target4) {
        Some(index) => println!("Found {} at index {}", target4, index),
        None => println!("{} not found", target4), // Expected: 100 not found
    }
    println!("---");

    // --- Test Case 5: Empty slice ---
    let target5 = 5;
    println!("Searching for {} in {:?}", target5, empty_vec);
    match binary_search(&empty_vec, target5) {
        Some(index) => println!("Found {} at index {}", target5, index),
        None => println!("{} not found", target5), // Expected: 5 not found
    }
    println!("---");

    // --- Using Rust's built-in binary search ---
    println!("Using standard library binary_search for {}:", target1);
    // Note: The standard library version requires a reference to the target
    match sorted_numbers.binary_search(&target1) {
        Ok(index) => println!("Found {} at index {}", target1, index), // Expected: Found 23 at index 5
        Err(insertion_index) => println!(
            "{} not found, would insert at index {}",
            target1, insertion_index
        ),
    }

    println!("Using standard library binary_search for {}:", target2);
    match sorted_numbers.binary_search(&target2) {
        Ok(index) => println!("Found {} at index {}", target2, index),
        Err(insertion_index) => println!(
            "{} not found, would insert at index {}",
            target2, insertion_index
        ), // Expected: 40 not found, would insert at index 7
    }
}
