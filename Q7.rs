/*
   PRoblem : Implement a function that returns the kth smallest element in a given array.


*/

/*
    Approach :
        1--> Sort the given array in non-decreasing order.

        2--> Return the kth Element: Return the element at index k - 1 from the sorted array.
*/

fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k == 0 || k > arr.len() {
        return None; // Return None if k is out of bounds
    }

    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();

    Some(sorted_arr[k - 1])
}

fn main() {
    let arr = vec![1, 5, 6, 3, 4, 6, 6];
    let k = 5;

    match kth_smallest(&arr, k) {
        Some(smallest) => println!("The {}th smallest element is {}", k, smallest),
        None => println!("k is out of bounds"),
    }
}
