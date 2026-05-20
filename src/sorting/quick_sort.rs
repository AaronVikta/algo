pub fn quick_sort(arr: &mut [i32]){
    if arr.len() <=1 {
        return;
    }

let pivot_idx = partition(arr);
//Recursively sort the two partitions
quick_sort(&mut arr[..pivot_idx]);
quick_sort(&mut arr[pivot_idx+1..]);
}

//Lumoto partition scheme - simpler to reasonabout
//Pivot:last element returns final pivot index

fn partition(arr: &mut [i32])-> usize{
    let n = arr.len();
let pivot = arr[n-1];
    let mut i = 0;
    for j in 0..(n-1){
        if arr[j] <= pivot{
            arr.swap(i,j);
            i +=1;
         }
        }
    arr.swap(i, n-1);
    i
}