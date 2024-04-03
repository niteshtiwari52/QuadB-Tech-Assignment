/*
   PRoblem :  Given a sorted array of integers, implement a function that returns the median of the array.

*/

/*
   APPROACH :
   1--> array of even length then the median is the average of the two middle elements.

   2--> array of odd lengrh then median is middle element

*/

fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        // If array has even number of elements
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
        (arr[mid_left] + arr[mid_right]) as f64 / 2.0
    } else {
        // If array has odd number of elements
        arr[len / 2] as f64
    }
}

fn main() {
    let sorted_array1 = vec![1, 2, 3, 4, 5];
    let sorted_array2 = vec![1, 2, 3, 4, 5, 6];

    println!(
        "Median of {:?}: {}",
        sorted_array1,
        find_median(&sorted_array1)
    );
    println!(
        "Median of {:?}: {}",
        sorted_array2,
        find_median(&sorted_array2)
    );
}
