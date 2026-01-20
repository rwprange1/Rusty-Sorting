use std::clone;

mod sort;

fn main() {
    println!("Hello, world!");
    let nums = sort::generate_data(5);

    let selection = sort::selection_sort(nums.clone());
    let bubble = sort::bubble_sort(nums.clone());
    let insertion = sort::insertion_sort(nums.clone());
    let merge = sort::merge_sort(nums.clone());
    println!("Selection Sort: {:?}\nBubble Sort: {:?}\nInsertion Sort: {:?}\nMerge Sort: {:?}", selection, bubble, insertion, merge);


    assert_eq!(selection, bubble);
    assert_eq!(selection, insertion);
    assert_eq!(insertion, merge);
}
