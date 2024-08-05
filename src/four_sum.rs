use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {

        let right_default = nums.len()-1;

        if right_default < 3 {return vec![]};

        let mut res: HashSet<Vec<i32>> = HashSet::new();

        nums.sort();

        for i in 0..nums.len()-3 {
            let ith = nums[i];

            if i > 0 && ith == nums[i - 1] {
                continue;
            }
            

            for j in i+1..nums.len()-2 {
                let jth = nums[j];

                let mut left = j+1;
                let mut right = right_default;

                
                'wh: while left < right {
                    
                    let num_l = nums[left];
                    let num_r = nums[right];

                    let mut result: i32 = 0;

                    for &number in &[ith, jth, num_l, num_r] {
                        let (new_result, overflowed) = result.overflowing_add(number);
                        if overflowed {
                            right -= 1;
                            continue 'wh;
                        }
                        result = new_result;
                    }

                    match result {
                        d if d < target => left += 1,
                        d if d > target => right -= 1,
                        d if d == target => {
                            res.insert(vec![ith, jth, num_l, num_r]);

                            while (left < right) && (nums[left] == num_l) {
                                left += 1;
                            }
                            while (left < right) && (nums[right] == num_r) {
                                right -= 1;
                            }

                        },
                        _ => unreachable!()
                    }
                }
            }
        }

        res.into_iter().collect()
    }


    // pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {

    //     if nums.len() < 4 {return vec![]};

    //     let mut res: HashSet<Vec<i32>> = HashSet::new();

    //     nums.sort();

    //     for i in 0..nums.len()-3 {

    //         if i > 0 && nums[i] == nums[i - 1] {
    //             continue;
    //         }
            

    //         for j in i+1..nums.len()-2 {

    //             let mut left = j+1;
    //             let mut right = nums.len()-1;

                
    //             'wh: while left < right {

    //                 let mut result: i32 = 0;

    //                 for &number in &[nums[i], nums[j], nums[left], nums[right]] {
    //                     let (new_result, overflowed) = result.overflowing_add(number);
    //                     if overflowed {
    //                         right -= 1;
    //                         continue 'wh;
    //                     }
    //                     result = new_result;
    //                 }

    //                 if result < target { 
    //                     left += 1; 
    //                 } else if result > target {
    //                     right -= 1; 
    //                 } else {
                        
    //                     res.insert(vec![nums[i], nums[j], nums[left], nums[right]]);

    //                     while (left < right) && (nums[left] == nums[left]) {
    //                         left += 1;
    //                     }
    //                     while (left < right) && (nums[right] == nums[right]) {
    //                         right -= 1;
    //                     }
    //                 }
    //             }
    //         }
    //     }

    //     res.into_iter().collect()
    // }
}