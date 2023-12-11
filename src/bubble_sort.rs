pub fn execute(arr: &mut [i32]) {
    for idx_1 in 0..arr.len() {
        let iter_end = arr.len() - idx_1 - 1;
        for idx_2 in 0..iter_end {
            if arr[idx_2] > arr[idx_2 + 1] {
                arr.swap(idx_2, idx_2 + 1);
            }
        }
    }
}
