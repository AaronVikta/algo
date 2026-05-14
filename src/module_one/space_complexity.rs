//space complexity examples

//O(1) space-auxiliary memory is constant regardless of n
#[allow(dead_code)]
pub fn sum (arr:&[i32])->i32{
    let mut total = 0;

    for &x in arr{
        total +=x;
    }
    total
}

//O(n) space - allocates a new array proportional to input size
#[allow(dead_code)]
pub fn doubld(arr: &[i32]) -> Vec<i32>{
    arr.iter().map(|x| x * 2).collect() //n new integers
}

//O(log n) space - recursive binary search
#[allow(dead_code)]
pub fn binary_search_rec(arr: &[i32], target: i32, lo: usize, hi:usize)->
Option<usize>{
    if lo >= hi{ return None;}

    let mid  = lo + (hi-lo)/2;

    match arr[mid].cmp(&target){
        std::cmp::Ordering::Equal => Some(mid),
        std::cmp::Ordering::Less => binary_search_rec(arr, target, mid+1, hi),
        std::cmp::Ordering::Greater => binary_search_rec(arr, target, lo, mid),
    }

}

//O(n) space -recursion to depth n 
//stack overflow rist for large in production
#[allow(dead_code)]
pub fn recursive_sum(arr: &[i32]) -> i32{
    if arr.is_empty(){
        return 0;
    }
    arr[0] + recursive_sum(&arr[1..])
}

//PROBLEM: Does this array contain any duplicate values?
//--Approach 1 O(n^2) time, O(1) space
#[allow(dead_code)]
pub fn has_duplicate_slow(arr: &[i32])->bool{
    for i in 0..arr.len(){
        for j in (i+1)..arr.len(){
            if arr[i] == arr[j]{
                return true;
            }
        }
        
    }
    false
}

//--Approach 2 O(n log n) time, O(n) space
#[allow(dead_code)]
//Trade memory for speed.Track seen values in a HashSet
pub fn has_duplicate_fast(arr: &[i32])->bool{
    let mut seen = std::collections::HashSet::new();

    for &x in arr {
        if !seen.insert(x){
            return true;
        }
    }
    false
}

/*
*At n = 1,000,000: 
Approach 1 = 500,000,000,000 ops. Approach 2 = 1,000,000 ops
The O(n) extra memory cost buys a 500,000x speedup
*/

#[allow(dead_code)]
//Example - What is the complexity?
pub fn mastery_a(n:usize){
    let mut i = n;

    while i>0{ //O(log n) time - i is halved each iteration
        let mut j = 0;

        while j < n{ //O(n) time - j goes from 0 to n-1
            j +=1;
        }
        i /=2; //i is halved each outer loop iteration
    }
}

#[allow(dead_code)]
pub fn mastery_b(n:usize){
    if n <= 1 {return;}
    mastery_b(n/2); //recurse on left half
    mastery_b(n/2); //recurse on right half
    //Do O(1) work here

}

