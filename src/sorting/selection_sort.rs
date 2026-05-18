pub fn selection_sort(arr: &mut [i32]){
    let n= arr.len();

    for i in 0..n{
        let min_idx = arr[i..]
        .iter()
        .enumerate()
        .min_by_key(|&(_, &val)| val)
        .map(|(idx, _)| idx + i) // adjust index to original array
        .unwrap_or(i); // default to current index if no minimum found

    if min_idx != i {
        arr.swap(i, min_idx);           
    }
}
}

pub fn selection_sort_generic<T: Ord> (arr: &mut [T]){
    let n = arr.len();

    for i in 0..n{
        let min_idx = (i..n).min_by_key(|&j| &arr[j])
        .unwrap_or(i);

    arr.swap(i, min_idx);
    }
}