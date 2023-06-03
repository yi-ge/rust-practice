// 摘水果
// https://leetcode.cn/problems/maximum-fruits-harvested-after-at-most-k-steps
// INLINE  ../../images/array/maximum_fruits_harvested_after_at_most_k_steps.jpeg

pub struct Solution;

impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        // 初始化左右指针和水果列表长度
        let mut left = 0;
        let mut right = 0;
        let n = fruits.len();
        // 初始化摘到的水果总数和最大值
        let mut sum = 0;
        let mut ans = 0;

        // 定义计算两个水果之间距离的闭包函数
        let step = |left: usize, right: usize| -> i32 {
            // 如果右侧水果在起始位置左边，则返回左侧水果位置到起始位置的距离
            if fruits[right][0] <= start_pos {
                start_pos - fruits[left][0]
            // 如果左侧水果在起始位置右边，则返回右侧水果位置到起始位置的距离
            } else if fruits[left][0] >= start_pos {
                fruits[right][0] - start_pos
            // 否则返回左右两侧水果到起始位置的距离之和再加上左右两侧水果的距离
            } else {
                std::cmp::min(
                    (start_pos - fruits[right][0]).abs(),
                    (start_pos - fruits[left][0]).abs(),
                ) + fruits[right][0]
                    - fruits[left][0]
            }
        };

        // 右指针遍历整个水果列表
        while right < n {
            // 累加摘到的水果数量
            sum += fruits[right][1];
            // 如果左右两侧水果之间距离超过了k，左指针向右移动并减去左侧水果的数量，直到距离小于等于k
            while left <= right && step(left, right) > k {
                sum -= fruits[left][1];
                left += 1;
            }
            // 更新最大值
            ans = std::cmp::max(ans, sum);
            // 右指针向右移动
            right += 1;
        }
        // 返回最大值
        ans
    }
}
