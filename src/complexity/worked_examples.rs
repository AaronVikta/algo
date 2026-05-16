fn example_a(arr: &[i32])->bool{
    for i in 0..arr.len(){
        for j in (i+1)..arr.len(){
            if arr[i]+ arr[j]==0{
                return true;
            }
        }
    }
    false
}

// the above function has a time complexity of O(n^2)
//Rule 2: Nested loops multiply their complexities

fn example_b(arr: &[i32], targets: &[i32]) -> Vec<bool> {
    let mut sorted = arr.to_vec();  
    sorted.sort();                  
    targets.iter().map(|t| {        
        sorted.binary_search(t).is_ok()
    }).collect()
}
//sorted is O(n log n)and map+ search is O(m log n) where m is the number of targets.
// So total complexity is O(n log n + m log n) 
//which simplifies to O((n+m) log n).

fn example_c(n: usize) {
    let mut i = n;
    while i > 0 {           // outer: halves each iteration → O(log n)
        let mut j = 0;
        while j < n {       // inner: always n iterations → O(n)
            println!("{} {}", i, j);
            j += 1;
        }
        i /= 2;
    }
}
// Rule 2 + Rule 4:
// Outer loop runs log n times (halving pattern)
// Inner loop runs n times per outer iteration
// Total: log n × n = O(n log n)