// 重新排列数组
// https://leetcode.cn/problems/shuffle-the-array/
// INLINE  ../../images/array/shuffle_the_array.jpeg

pub struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        nums.iter()
            .zip(nums.iter().skip(n as usize))
            .flat_map(|(&x, &y)| [x, y])
            .collect()
    }
}
