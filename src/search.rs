


pub fn binary_search(nums: &Vec<i64>, val: i64) -> Option<usize>{



    let mut low = 0;
    let mut mid;
    let mut high = nums.len() - 1;

    while low  <= high {
        mid = low + (high- low)/2;

        if nums[mid] == val{
            return Some(mid);
        }

        if nums[mid] < val {
            low = mid + 1;

        }else {
            if high  == 0 {
                break;
            }
            high = mid -1;
        }

    }
    None
}

pub fn linear_search(nums: &Vec<i64>, val: i64) -> Option<usize>{
    for i in 0..nums.len(){
        if nums[i] == val {
            return Some(i);
        }
    }
    None
}