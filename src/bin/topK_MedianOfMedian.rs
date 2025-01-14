mod top_k {
    /// Finds the kth smallest element in the subarray from index l to r in the given array.
    ///
    /// # Arguments
    ///
    /// * `arr`: A mutable reference to the vector of integers.
    /// * `l`: The starting index of the subarray.
    /// * `r`: The ending index of the subarray.
    /// * `k`: The index of the element to be found.
    ///
    /// # Returns
    ///
    /// * The kth smallest element in the specified subarray.
    pub fn bfprt(arr: &mut Vec<i32>, l: usize, r: usize, k: usize) -> i32 {
        if l == r {
            return arr[l];
        }
        let pivot = median_of_medians(arr, l, r);
        let pos = partition(arr, l, r, pivot);
        if l + k >= pos[0] && l + k <= pos[1] {
            return arr[l + k];
        }
        if l + k < pos[0] {
            return bfprt(arr, l, pos[0] - 1, k);
        }
        bfprt(arr, pos[1] + 1, r, k + l - pos[1] - 1)
    }
/// Finds the approximate median of the array `arr` between indices `l` and `r`
/// using the Median of Medians algorithm. This method partitions the array
/// into groups of five elements, computes the median of each group, and then
/// recursively finds the median of these medians.
///
/// # Arguments
///
/// * `arr` - A mutable reference to the vector of integers.
/// * `l` - The starting index of the subarray.
/// * `r` - The ending index of the subarray.
///
/// # Returns
///
/// * The approximate median value of the specified subarray.

    fn median_of_medians(arr: &mut Vec<i32>, l: usize, r: usize) -> i32 {
        let mut n = r - l + 1;
        let group_size = 5;
        let mut medians = vec![0; n / group_size + if n % group_size > 0 { 1 } else { 0 }];
        for i in 0..medians.len() {
            medians[i] = compute_median(arr, l + i * group_size, r.min(l + i * group_size + group_size - 1));
        }
        n = medians.len();
        bfprt(&mut medians, 0, n - 1, n / 2)
    }
    /// Computes the median of the given subarray `arr` between indices `l` and `r`.
    ///
    /// # Arguments
    ///
    /// * `arr` - A mutable reference to the vector of integers.
    /// * `l` - The starting index of the subarray.
    /// * `r` - The ending index of the subarray.
    ///
    /// # Returns
    ///
    /// * The median value of the specified subarray.
    fn compute_median(arr: &mut Vec<i32>, l: usize, r: usize) -> i32{
        let slice = &mut arr[l..=r];
        slice.sort();
        slice[slice.len() / 2]
    }
    /// Partitions the array `arr` in place around the `pivot` value, such that all elements
    /// less than the pivot come before all elements equal to the pivot, which come before
    /// all elements greater than the pivot.
    ///
    /// # Arguments
    ///
    /// * `arr` - A mutable reference to the vector of integers.
    /// * `l` - The starting index of the subarray.
    /// * `r` - The ending index of the subarray.
    /// * `pivot` - The pivot value to partition the array around.
    ///
    /// # Returns
    ///
    /// * A vector containing two indices: the start and end of the subarray where all elements
    ///   are equal to the pivot value.

    fn partition(arr: &mut Vec<i32>, l: usize, r: usize, pivot: i32) -> Vec<usize> {
        let mut small_st = l as i32 - 1;
        let mut equal_len = 0;
        for j in l..=r {
            if arr[j] < pivot {
                small_st += 1;
                arr.swap(j as usize, small_st as usize);
                if equal_len > 0 {
                    arr.swap(small_st as usize, (small_st + equal_len) as usize);
                }
                continue;
            }
            if arr[j] == pivot {
                equal_len += 1;
                arr.swap(j as usize, (small_st + equal_len) as usize);
            }
        }
        vec![small_st as usize + 1, small_st as usize + equal_len as usize]
    }
}
fn main() {
    let mut arr = vec![1, 2, 3, 4, 5, 6, 6, 6, 6, 6, 7, 8, 9, 10];
    let n = arr.len();
    println!("{}", top_k::bfprt(&mut arr, 0, n - 1, 8));
}
