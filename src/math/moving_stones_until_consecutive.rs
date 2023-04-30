// 移动石子直到连续
// https://leetcode.cn/problems/moving-stones-until-consecutive
// INLINE  ../../images/math/moving_stones_until_consecutive.jpeg

pub struct Solution;

impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut stones = vec![a, b, c];
        stones.sort();
        let min = if stones[2] - stones[0] == 2 {
            0
        } else if stones[1] - stones[0] <= 2 || stones[2] - stones[1] <= 2 {
            1
        } else {
            #[cfg_attr(tarpaulin, skip)]
            2
        };
        let max = stones[2] - stones[0] - 2;
        vec![min, max]
    }
}
