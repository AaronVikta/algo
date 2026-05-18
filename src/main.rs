mod complexity;
use complexity::time_complexity::bubble_sort;

mod sorting;
use sorting::{bubble_sort::bubble_sort_2, insertion_sort::insertion_sort};



fn main() {
    // let arr_a = [1,2,3];
    // let arr_b= [4,5];

    // compare_all_pairs(&arr_a, &arr_b);
    //     let arr_c = [1,2,3,4];
    //     all_pairs_same(&arr_c);
    //     process_both(&arr_a, &arr_b);
    //     let pairs = cartesian_product(&arr_a, &arr_b);
    //     println!("Cartesian product: {:?}", pairs);

    //     let target = 3;
    //     binary_search(&arr_a, &target);

    //     linear_search(&arr_c, target);  
    //     find_max(&arr_c);
    let mut arr = vec![5,2,9,1,5,6];
    println!("Original array: {:?}", arr);

    bubble_sort(&mut arr);

    println!("Sorted array: {:?}", arr);
let mut arr2 = vec![5,2,9,1,5,6];
    println!("Original array: {:?}", arr2);

    insertion_sort(&mut arr2);

    println!("Sorted array: {:?}", arr2);
}


