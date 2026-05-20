pub fn merge_sort(arr: &mut Vec<i32>){
    let n = arr.len();

    if n <= 1{
        return;
    }

    let mid = n/2;

    //Divide: Split into two halves 
    //to_vec() allocates 0(n/2) extra space per level - total = O(n) space

    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    //conquer: Recursively sort both halves
    merge_sort(&mut left);
    merge_sort(&mut right);

    //Combine - merge sorted halves back into original array
    merge(arr, &left, &right);
}


fn merge(arr: &mut Vec<i32>, left: &[i32], right: &[i32]){
    let (mut i, mut j,mut k) = (0,0,0);

    // compare front of each half; take the smaller element
    //stable left[i] <= right[j] means left wins ties -preserve order

    while i <left.len() && j <right.len(){
        if left[i] <= right[j]{
            arr[k] = left[i];
            i +=1;
        } else{
            arr[k] = right[j];
            j+=1;
        }
        k+=1;
    }

    //Drain remaining elements at most one side has leftovers
    while i<left.len(){arr[k] = left[i]; i +=1; k +=1;}
    while j<right.len(){arr[k] = right[j]; j +=1; k +=1;}
}