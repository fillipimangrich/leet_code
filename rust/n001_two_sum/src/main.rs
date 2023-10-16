use std::collections::HashMap;

struct Solution1;
struct Solution2;

//dumb solution
impl Solution1 {
    fn two_sum(nums: Vec<i32>, target:i32) -> Vec<i32>{
        for (i, num_a) in nums.iter().enumerate(){
            for (j, num_b) in nums.iter().enumerate(){
                if i == j{
                    continue;
                }  
                if num_a+num_b == target{
                    return vec![i as i32,j as i32];
                }
            }
        }
        vec![]
    }
}
impl Solution2 {
    fn two_sum(nums: Vec<i32>, target:i32) -> Vec<i32>{
        let mut hash_map: HashMap<i32, usize> = HashMap::new();
        for (i, num) in nums.iter().enumerate(){
            let complement = target - num;
            if let Some(&index) = hash_map.get(&complement){
                return vec![index as i32, i as i32]
            }
            hash_map.insert(*num, i);
        }
        vec![]
    }
}

fn main() {
    Solution2::two_sum(vec![2,7,11,15], 9);
}
