// For homework 6

fn running_sum(nums: Vec<i32>) -> Vec<i32> {  
    let mut new_nums : Vec<i32> = vec![]; 
    for i in 0..nums.len() {
        let mut sum : i32 = nums[i]; 
        for a in 0.. i {
            sum += nums[a as usize]            
        }
        new_nums.push(sum)
    }
    
    new_nums
}  
  
fn main() {  
    println!("{:?}", running_sum(vec![1, 1, 1, 1, 1]));
}