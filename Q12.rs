/*
   PRoblem : Find the maximum subarray sum in Rust


*/

/*
    Approach : Kadane's algorithm
    1--> initialize max sum & curr sum with arr[0] .
    2--> iterate each elemnts and update current sum and max summ
    3--> return max sum.

*/

fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_sum = arr[0];
    let mut current_sum = arr[0];

    for &num in arr.iter().skip(1) {
        current_sum = current_sum.max(current_sum + num);
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}

fn main() {
    let arr = vec![1, 2, 3, 6, 7, 8];
    let max_sum = max_subarray_sum(&arr);
    println!("Maximum subarray sum: {}", max_sum);
}
