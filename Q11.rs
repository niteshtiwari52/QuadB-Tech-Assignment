/*
   PRoblem : Merge two sorted arrays in Rust

*/

/*
    Approach :
  1--> Create an empty vector to store the merged array.
  2--> Use two pointers, one for each array, to iterate through both arrays simultaneously.
  3--> Compare elements pointed to by the pointers.
  4--> Insert the smaller element into the merged vector and advance the corresponding pointer.
  5--> After merging, if any elements remain in either array, add them to the merged vector.
  5--> Return the merged vector as the result.
*/

fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);

    // Merge the arrays by comparing elements and inserting them into the merged array
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }

    // Add remaining elements from arr1, if any
    while i < arr1.len() {
        merged.push(arr1[i]);
        i += 1;
    }

    // Add remaining elements from arr2, if any
    while j < arr2.len() {
        merged.push(arr2[j]);
        j += 1;
    }

    merged
}

fn main() {
    let arr1 = vec![1, 3, 5, 7, 9];
    let arr2 = vec![2, 4, 6, 8, 10];

    let merged = merge_sorted_arrays(&arr1, &arr2);

    println!("Merged array: {:?}", merged);
}
