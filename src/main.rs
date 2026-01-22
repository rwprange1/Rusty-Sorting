

mod sort;
mod search;

use std::time::Instant;

use rand::Rng;

use crate::{search::binary_search, sort::quick_sort};


fn main() {
    loop {
        test_binary(10);
    }
}

fn test_binary(len: usize){

    let mut nums = sort::generate_data(len);
    let mut rng = rand::rng();

    quick_sort(&mut nums, 0, len-1);
    let num = rng.random_range(0..=(10*len));
    let mut data = num as i64;
    if num < nums.len(){
        data = nums[num];
    }


    println!("{:?} and looking for {}", nums, data);
    match binary_search(&nums, data) {
        Some(n) => {
            if nums[n] != data {
                println!("{} != {}\nData: {:?}", nums[n], data, nums);
            }
        },
        None => {
            println!("{} was not in {:?}", data, nums);
        }
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