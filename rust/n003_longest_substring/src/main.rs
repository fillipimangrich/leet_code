use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest: i32 = 0;
        let mut possible: i32 = 0;
        let mut hash_map: HashMap<char, i32> = HashMap::new();
        let mut index: i32 = 0;
        let mut a: char;

        if s.len() == 1{
            return 1;
        }

        while index < s.len() as i32{

            a = s.as_bytes()[index as usize] as char;

            if let Some(&new_index) =  hash_map.get(&a){
                
                hash_map.clear();

                if possible > longest
                {
                    longest = possible;
                }

                possible = 0;

                index = new_index;

                a = s.as_bytes()[index as usize] as char;
            }

            if index == s.len().try_into().unwrap(){
                break
            }

            index += 1;

            possible += 1;

            hash_map.insert(a, index);

        }

        if longest < possible{
            return possible;
        }

        longest
    }
}

fn main(){
    let s = String::from("aab");
    
    let length = Solution::length_of_longest_substring(s);

    println!("{}",length);
}