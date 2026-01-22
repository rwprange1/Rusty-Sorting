use rand::prelude::*;



pub fn selection_sort(nums:  Vec<i64>) -> Vec<i64>{
    let mut copy = nums.clone();
    for i in 0..copy.len(){
        let mut min_index= i;
        for j in i..copy.len(){
          if copy[j] < copy[min_index] {
                min_index = j;
          }
        }
        let temp = copy[i];
        copy[i] = copy[min_index];
        copy[min_index] = temp;
    }
    copy
}

pub fn bubble_sort(nums: Vec<i64>) -> Vec<i64>{
    let mut copy = nums.clone();
  
    let mut sorted = false;
    let mut i;
    let mut passes = 0;
    while !sorted {
        sorted = true;
        i = 0;
        let mut j = 1;
        while j < nums.len()-passes{
            if copy[i] > copy[j] {
                sorted = false;
                let temp = copy[i];
                copy[i] = copy[j];
                copy[j] = temp;
            }   
            i+=1;
            j+=1;

        }
        passes += 1;

    }
    copy
}

pub fn insertion_sort(nums: Vec<i64>) -> Vec<i64>{
    let mut copy = nums.clone();
    for i in 1..nums.len(){
        let mut temp = i;
        let mut j = i-1;

        while copy[temp] < copy[j]{
            let val = copy[temp];
            copy[temp] = copy[j];
            copy[j] = val;

            if j == 0 {
                break;
            }else {
                temp -= 1;
                j -= 1;
            }
            
        }

    }
    copy
}

pub fn merge_sort(nums: Vec<i64>) -> Vec<i64>{
    if nums.len() == 1{
        return nums
    }

    let mid = nums.len() /2;
    
    let left = nums[0..mid].to_vec();
    let right = nums[mid..].to_vec();

    let left_merge = merge_sort(left);
    let right_merge= merge_sort(right); 

    
    merge(left_merge, right_merge)

}

fn merge(left: Vec<i64>, right: Vec<i64>) -> Vec<i64> {
    let mut to_ret = Vec::new();

    let mut left_p = 0;
    let mut right_p = 0;
    while left_p < left.len() && right_p < right.len() {
        if left[left_p] < right[right_p] {
          to_ret.push(left[left_p]);
          left_p += 1;  
        }else{
            to_ret.push(right[right_p]);
            right_p += 1;
        }
    }

    if left_p < left.len(){
        to_ret.extend(left[left_p..].to_vec());
    }else{
        to_ret.extend(right[right_p..].to_vec())
    }


    to_ret

}


pub fn quick_sort(nums: &mut Vec<i64>, low: usize, high:usize){

    if low < high {
        let mut rng = rand::rng();
    
   
        let pivot: usize = rng.random_range(low..high);
        let pi = hoare_partition(nums,  low, high, pivot);
   
        quick_sort(nums, low, pi);
        quick_sort(nums, pi + 1, high);

    }

}

fn hoare_partition(nums:  &mut Vec<i64>, low:  usize, high: usize, pivot: usize) -> usize{
    let val = nums[pivot];
    let mut i:i64 = low as i64 - 1;
    let mut j = high + 1;
    loop {
        
        i += 1;
        while nums[i as usize] < val {
            i += 1;
        }

        j -= 1;
        while nums[j] > val {
            j -= 1;
        }

        if i as usize >= j {
            return j;
        }
        let temp = nums[i as usize];
        nums[i as usize] = nums[j];
        nums[j] = temp;
    }
    


}




pub fn generate_data(size: usize) -> Vec<i64>{
    // Get an RNG:
    let mut rng = rand::rng();
    let mut nums = Vec::new();

    for _i in 0..size{
        nums.push(rng.random_range(-1000..1000));
    }

    nums

}

