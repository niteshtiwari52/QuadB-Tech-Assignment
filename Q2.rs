/*
PROBLEM : Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

*/

/**
   Approach : Binary search Algorithm

*/
fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;
    let mut result = None;

    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid] == target {
            result = Some(mid);
            high = mid - 1;
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    result
}

fn main() {
    let sorted_array = vec![100, 123, 123, 150, 150, 150, 200];
    let target_number = 200;
    match first_occurrence(&sorted_array, target_number) {
        Some(index) => println!("Index of {}: {}", target_number, index),
        None => println!("{} not found in the array", target_number),
    }
}
