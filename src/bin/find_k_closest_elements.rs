pub struct Solution;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let len = arr.len();
        let k = k as usize;

        let mut left = len/2 - k/2;
        let mut right = left + k - 1;

        // println!("{}___{}", left, right);

        while right < len-1 && (arr[left] - x).abs() >= (arr[right+1] - x).abs() {
            left += 1;
            right += 1;
        }
        
        
        while left > 0 && (arr[left-1] - x).abs() <= (arr[right] - x).abs() {
            //* change to mid of left part for log(n)
            left -= 1;
            right -= 1;
        }

        (arr[left..=right]).to_vec()
    }
}

fn main() {
    let arr: Vec<i32> = [1,2,3,4,5].to_vec();
    let (k, x) = (4, 3);
    println!("{:?}", Solution::find_closest_elements(arr, k, x));
}




