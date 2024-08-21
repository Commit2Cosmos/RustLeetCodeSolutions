
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn trap_n_memory(height: Vec<i32>) -> i32 {

        //* calculate max height to the left */
        let mut max_height = 0;
        let mut max_left_height = height.iter().map(|&x| {
            max_height = x.max(max_height);
            max_height
        })
        .collect::<Vec<i32>>();

        max_left_height.insert(0, 0);

        //* calculate max height to the right */
        let mut max_height = 0;
        let max_right_height: Vec<i32> = height[1..].iter().rev().map(|&x| {
            max_height = x.max(max_height);
            max_height
        })
        .collect::<Vec<i32>>();


        height.iter().zip(max_left_height.iter().zip(max_right_height.iter().rev())).map(|(h, (l, r))| {
            0.max(l.min(r)-h)
        })
        .sum()

    }


    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut left_max = height[left];

        let mut right: usize = height.len()-1;
        let mut right_max = height[right];

        let mut tot = 0;

        while left < right {
            if left_max < right_max {
                left += 1;
                tot += 0.max(left_max - height[left]);
                left_max = left_max.max(height[left]);
            } else {
                right -= 1;
                tot += 0.max(right_max - height[right]);
                right_max = right_max.max(height[right]);
            }
        }

        tot
    }
}