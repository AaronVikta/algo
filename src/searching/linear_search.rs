fn linear_search(arr:&[i32], target: i32)->Option<usize>{
    for (i, &val) in arr.iter().enumerate(){
        if val == target{
            return Some(i);
        }
    }
    None
}

fn linear_search_generic<T: PartialEq>
(arr: &[T], target: &T)->Option<usize>{
    arr.iter().position(|x| x == target) 
}