pub fn execute(arr: &mut [i32]) {
    for idx in 0..arr.len() {
        let mut min_idx = idx;
        for idx_2 in idx..arr.len() {
            if arr[min_idx] > arr[idx_2] {
                min_idx = idx_2;
            }
        }
        if min_idx != idx {
            arr.swap(idx, min_idx)
        }
    }
}
