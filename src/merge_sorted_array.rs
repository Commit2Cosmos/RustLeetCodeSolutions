pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut merge_point = (m + n - 1) as usize;
        let mut point1 = m-1;
        let mut point2 = n-1;

        while point2 >= 0 {
            if point1 >= 0 && nums1[point1 as usize] > nums2[point2 as usize] {
                nums1[merge_point] = nums1[point1 as usize];
                point1 -= 1;
            } else {
                nums1[merge_point] = nums2[point2 as usize];
                point2 -= 1;
            }
            merge_point = merge_point.wrapping_sub(1);
        }
    }
}