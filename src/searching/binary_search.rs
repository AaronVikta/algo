fn binary_search_manual<T:Ord>(arr: &[T], target: &T)-> Option<usize>{
    let mut lo = 0;
    let mut hi = arr.len();

    //LOOP INVARIANT 
    /*
    if target is in arr,it isin arr[lo..hi] exclusive hi
    //initialization arr[0..arr.len()] is the whole array
    -trivially holds 
     */
    while lo < hi{
        //CRITICAL: Overflow safe midpoint calculation
        let mid = lo + (hi -lo)/2;

        match arr[mid].cmp(target){
            std::cmp::Ordering::Equal =>{
                return Some(mid);
            }

            std::cmp::Ordering::Less =>{
                lo = mid+1;
            }

            std::cmp::Ordering::Greater =>{
                hi = mid;
            }
        }
    }
    None
}

//find first occurrence 
fn find_first<T:Ord>(arr: &[T], target: &T)->Option<usize>{
    let mut lo = 0;
    let mut hi = arr.len();
    let mut result = None;

    while lo<hi{
        let mid = lo + (hi-lo)/2;

        match arr[mid].cmp(target){
            std::cmp::Ordering::Equal =>{
                result = Some(mid);//Record this candidate
                hi = mid; //but keep searching in the left for earlier
            }

            std::cmp::Ordering::Less =>{
                lo = mid+1;
            }
            std::cmp::Ordering::Greater =>{
                hi = mid;
            }
        }
    }
    result

}

fn lower_point<T:Ord>(arr:&[T], target:&T) ->usize{
    arr.partition_point(|x| x<target)
}

fn upper_point<T:Ord>(arr:&[T], target:&T) ->usize{
    arr.partition_point(|x| x<=target)
}

fn search_rotated(arr:&[i32], target:i32)->Option<usize>{
    let mut lo = 0;
    let mut hi = arr.len();

    while lo<hi{
        let mid = lo+ (hi-lo)/2;

        if arr[mid] == target{
            return Some(mid);
        }

        //KEY INSIGHT: at least one half is always fully sorted
        //Determine which half is sorted by comparing 
        //arr[lo] and arr[mid]

        if arr[lo] <= arr[mid]{
            //left half arr[lo..=mid] is sorted
            if arr[lo] <= target && target < arr[mid]{
                hi = mid;
            }
            else{
                lo = mid +1;
            }
        }
        else{
            if arr[mid] <target && target <= arr[hi-1]{
                lo = mid + 1;
            }
            else{
                hi = mid;
            }
        }
    }
    None
}