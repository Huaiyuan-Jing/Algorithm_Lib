mod top_k {
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
    fn compute_median(arr: &mut Vec<i32>, l: usize, r: usize) -> i32{
        let slice = &mut arr[l..=r];
        slice.sort();
        slice[slice.len() / 2]
    }
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
