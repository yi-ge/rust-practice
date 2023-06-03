// 移动石子直到连续 II
// https://leetcode.cn/problems/moving-stones-until-consecutive-ii
// INLINE  ../../images/sort/moving_stones_until_consecutive_ii.jpeg

pub struct Solution;

impl Solution {
    pub fn num_moves_stones_ii(stones: Vec<i32>) -> Vec<i32> {
        // 将石头位置排序
        let mut stones = stones;
        stones.sort();
        // 石头数量
        let n = stones.len();
        // 最大移动次数
        let max = std::cmp::max(
            stones[n - 1] - stones[1] - n as i32 + 2,
            stones[n - 2] - stones[0] - n as i32 + 2,
        );
        // 最小移动次数
        let mut min = max;
        // 左右指针 i, j
        let mut i = 0;
        let mut j = 0;
        while j < n {
            // 当前区间的长度大于石头数量时，左指针右移
            while stones[j] - stones[i] + 1 > n as i32 {
                i += 1;
            }
            // 如果当前区间只剩下一个空位，并且空位与两端的石头相邻，那么最小移动次数为2
            if j - i + 1 == n - 1 && stones[j] - stones[i] + 1 == n as i32 - 1 {
                min = std::cmp::min(min, 2);
            } else {
                // 否则最小移动次数为区间内没有被占据的空位数量
                min = std::cmp::min(min, n as i32 - (j - i + 1) as i32);
            }
            j += 1;
        }
        vec![min, max]
    }
}
