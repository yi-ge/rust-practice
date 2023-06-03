// 移动石子直到连续
// https://leetcode.cn/problems/moving-stones-until-consecutive
// INLINE  ../../images/math/moving_stones_until_consecutive.jpeg

pub struct Solution;

impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        // 将三个石头按从小到大的顺序存入stones数组
        let mut stones = vec![a, b, c];
        stones.sort();
        // 计算最小移动次数
        let min = if stones[2] - stones[0] == 2 {
            // 如果三个石头本来就连续，不需要移动，最小移动次数为0
            0
        } else if stones[1] - stones[0] <= 2 || stones[2] - stones[1] <= 2 {
            // 如果最小距离小于等于2，则只需要移动一颗石头，最小移动次数为1
            1
        } else {
            #[cfg_attr(tarpaulin, skip)]
            2 // 否则需要移动两颗石头，最小移动次数为2
        };
        // 计算最大移动次数
        let max = stones[2] - stones[0] - 2; // 最大移动次数等于两端石头中间隔了几个空位
        vec![min, max]
    }
}
