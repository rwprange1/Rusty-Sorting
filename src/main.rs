

mod sort;
use std::time::Instant;


fn main() {
    for i in 6..=10{
        let num_elements = (10 as i128).pow(i);
        println!("Testing run times for  {} elements", num_elements);
        let nums = sort::generate_data(num_elements as usize);
    

        let merge = test_merge(nums.clone());
        let quick = test_quick(nums.clone());    

        let selection = test_selection(nums.clone());
        let bubble = test_bubble(nums.clone());
        let insertion = test_insertion(nums.clone());
        
        assert_eq!(selection, bubble);
        assert_eq!(selection, insertion);
        assert_eq!(insertion, merge);
        assert_eq!(merge, quick);

        println!("Test Case Done\n\n");
    
    }
    
}


fn test_merge(nums: Vec<i64>) -> Vec<i64>{
    let time = Instant::now();
    let merge = sort::merge_sort(nums.clone());
    let end = time.elapsed();
    println!("Merge Sort finished in {:?}", end);
    merge
}

fn test_quick(nums: Vec<i64>) -> Vec<i64>{
    let mut quick = nums.clone();
    let time = Instant::now();
    sort::quick_sort(&mut quick, 0, nums.len()-1);
    let end = time.elapsed();
    println!("Quick Sort finished in {:?}", end);
    quick
}

fn test_selection(nums: Vec<i64>) -> Vec<i64>{
    let time = Instant::now();
    let selection = sort::selection_sort(nums.clone());
    let end = time.elapsed();
    println!("Selection Sort finished in {:?}", end);
    selection
}

fn test_bubble(nums: Vec<i64>) -> Vec<i64>{
    let time = Instant::now();
    let bubble = sort::bubble_sort(nums.clone());
    let end = time.elapsed();
    println!("Bubble Sort finished in {:?}", end);
    bubble
}

fn test_insertion(nums: Vec<i64>) -> Vec<i64>{
    let time = Instant::now();
    let insertion = sort::insertion_sort(nums.clone());
    let end = time.elapsed();
    println!("Insertion Sort finished in {:?}", end);
    insertion
}