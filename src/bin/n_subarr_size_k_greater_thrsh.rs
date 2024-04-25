pub struct Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {

        let k = k as usize;

        let mut sum = arr[0..k].iter().sum::<i32>();
        let mut res: i32 = if sum/k as i32 >= threshold { 1 } else { 0 };

        for i in k..arr.len() {
            sum += arr[i] - arr[i-k];
            
            if sum/k as i32 >= threshold {
                res += 1;
            }
            
            // println!("{:?}", arr[i-k+1..=i].to_vec());
            // println!("{:?}", sum/k as i32);
            // println!("------------------------");
        }

        res
    }
}

fn main() {
    let arr: Vec<i32> = [2,2,2,2,5,5,5,8].to_vec();
    let k: i32 = 3;
    let threshold: i32 = 4;
    println!("{:?}", Solution::num_of_subarrays(arr, k, threshold));
}