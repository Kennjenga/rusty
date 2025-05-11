// list, search, loop while left <= right, 

pub fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() as i32 - 1;

    while left <= right {
        let mid = (left + right) / 2;
        if arr[mid as usize] == target {
            return Some(mid as usize);
        } else if arr[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    None
}
