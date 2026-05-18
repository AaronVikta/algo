pub fn insertion_sort(arr: &mut [i32]){

 // Start at index 1: arr[0..1] is trivially sorted
 for i in 1..arr.len(){
    let key = arr[i];
    let mut j = i;

    //shif elements greater than key one position to the right
    // this creates the gap for the key to be inserted
    while j>0 && arr[j-1] > key{
        arr[j]= arr[j-1];
        j -=1;
    }
    arr[j] = key;
 }   
}


//Generic version using Ord trait
pub fn insertion_sort_generic<T: Ord + Clone>(arr: &mut [T]){
    for i in 1..arr.len(){
let key = arr[i].clone();

let mut j =i;

while j >0 && arr[j-1]>key{
    arr[j] = arr[j-1].clone();

    j -=1;
}
    }
}

//Binary insertion sort: uses binary search to find the correct position for the key
pub fn binary_insertion_sort<T:Ord>(arr: &mut [T]){
    for i in 1..arr.len(){
        //Binary search the sorted prefix for insertion point
        let pos = arr[..i].partition_point(|x|x <= &arr[i]);

        //Rotate shifts arr[pos..=i] right by one - O(i)
        arr[pos..=i].rotate_right(1);
    }
}