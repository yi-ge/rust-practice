// 移动石子直到连续 II
// https://leetcode.cn/problems/moving-stones-until-consecutive-ii
// INLINE  ../../images/sort/moving_stones_until_consecutive_ii.jpeg

pub struct Solution;

impl Solution {
    pub fn num_moves_stones_ii(stones: Vec<i32>) -> Vec<i32> {
        let mut stones = stones;
        stones.sort();
        let n = stones.len();
        let max = std::cmp::max(
            stones[n - 1] - stones[1] - n as i32 + 2,
            stones[n - 2] - stones[0] - n as i32 + 2,
        );
        let mut min = max;
        let mut i = 0;
        let mut j = 0;
        while j < n {
            while stones[j] - stones[i] + 1 > n as i32 {
                i += 1;
            }
            if j - i + 1 == n - 1 && stones[j] - stones[i] + 1 == n as i32 - 1 {
                min = std::cmp::min(min, 2);
            } else {
                min = std::cmp::min(min, n as i32 - (j - i + 1) as i32);
            }
            j += 1;
        }
        vec![min, max]
    }
}
