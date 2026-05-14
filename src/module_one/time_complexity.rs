

/*
RULE 3 Different inputs get different variables, 
and we multiply their sizes to get the total complexity.
*/
//This is NOT O(n2) - two different input sizes
#[allow(dead_code)]
pub fn compare_all_pairs(arr_a: &[i32], arr_b:&[i32]){
    //arr_a has a length 'a' and arr_b has a length 'b'
    for &x in arr_a{
        for &y in arr_b{
            println!("Comparing {} and {}", x, y);
        }
    }
    //Complexity : O(a*b), not O(n2) 
    //Only O(n2) if arr_a.len() == arr_b.len() == n is guaranteed
}

#[allow(dead_code)]
//contrast with this function which is O(n2) - same input size
 pub fn all_pairs_same(arr:&[i32]){
    for i in 0..arr.len(){
        for j in (1+1)..arr.len(){
            println!("Comparing {} and {}", arr[i], arr[j]);
        }
    }

    //Here both loops reference the same input of length n -> O(n2)
 }

 /*
 RULE 4: Sequential blocks add, nested blocks multiply

  */
#[allow(dead_code)]
  pub fn process_both(arr_a:&[i32],arr_b:&[i32])-> (i32,i32){
    let sum_a: i32 = arr_a.iter().sum();
    let sum_b: i32 = arr_b.iter().sum();

    (sum_a, sum_b)
    //Complexity: O(a + b) - sequential blocks add
  }

  #[allow(dead_code)]
  pub  fn cartesian_product(arr_a: &[i32], arr_b:&[i32])->Vec<(i32,i32)>{
    let mut pairs = Vec::new();
    for &x in arr_a{
        for &y in arr_b{
            pairs.push((x,y));
        }
    }
    pairs
  }

  //Binary Search O(log n)
  #[allow(dead_code)]
  pub fn binary_search<T:Ord>(arr:&[T], target: &T) ->Option<usize>{
    let mut lo = 0;
    let mut hi = arr.len(); //exclusive upper bound

    while lo < hi{
        //CRITICAL use hi -lo not lo + hi
        //lo + hi can overflow if both are new usize::MAX

        let mid = lo + (hi -lo)/2;

        match arr[mid].cmp(target){
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less=> lo = mid+1,
            std::cmp::Ordering::Greater =>hi =mid,
        }

        //Each iteration: Search space halves -> O(log n)
    }
    None
  }

  //Linear Time Example O(n)
#[allow(dead_code)]
  pub fn find_max(arr: &[i32])->Option<i32>{
    let mut max: Option<i32> = None;
    for &val in arr{
        max = Some(match max{
            None=>val,
            Some(current) => current.max(val)
        });
    }
    max
  }

  //Linear search O(n)
  #[allow(dead_code)]
  pub fn linear_search(arr:&[i32], target:i32)->Option<usize>{
    for (i, &val) in arr.iter().enumerate(){
        if val == target{
            return Some(i);
        }
    }
    None
  }

  #[allow(dead_code)]
  //Merge Sort
  pub fn merge_sort(arr: &mut Vec<i32>){
    let n = arr.len();
    if n <= 1 { return;}

    let mid = n/2;

    let mut left = arr[..mid].to_vec();//O(n) space: left half copy

    let mut right = arr[mid..].to_vec(); //O(n) space:right half copy

    merge_sort(&mut left); //recurse on left - depth log n
    merge_sort(&mut right);//recurse on right - depth log n

    merge(arr, &left, &right); //merge two sorted halves -> O(n)
    
  }

  pub fn merge(arr: &mut Vec<i32>, left: &[i32], right:&[i32]){
    let (mut i, mut j, mut k) =(0,0,0);

while i <left.len() && j< right.len(){

    if left[i] <= right[j] {arr[k]= left[i]; i +=1;}
    else {arr[k] = right[j]; j+=1;}

    k +=1;
    }
    while i <left.len() {arr[k] = left[i]; i+=1; k+=1;}
    while j < right.len() {arr[k] = right[j]; j+=1; k+=1;}
}

//WHY O(n log n) ?
//Merge sort divides the array into halves log n times, and each merge step takes O(n
//time to combine the halves. So total complexity is O(n log n).


//O(n2) Quadratic time example: Bubble Sort
//This is a simple implementation of bubble sort, 
//which has a worst-case time complexity of O(n^2) 
//due to the nested loops. 
// It quickly becomes inefficient for large input sizes,
// making it impractical for sorting large datasets.

pub fn bubble_sort(arr: &mut Vec<i32>){
    let n = arr.len();

    for i in 0..n{
        for j in 0..n-i-1{
            if arr[j] >arr[j+1]{
                arr.swap(j, j+1);
            }
        }
        //Total comparisons (n-1) + (n-2) + ... + 1 = n(n-1)/2 -> O(n2)
    }
} 

