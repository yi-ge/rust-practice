// 摘水果
// https://leetcode.cn/problems/maximum-fruits-harvested-after-at-most-k-steps
// INLINE  ../../images/array/maximum_fruits_harvested_after_at_most_k_steps.jpeg

pub struct Solution;

impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let n = fruits.len();
        let mut sum = 0;
        let mut ans = 0;

        let step = |left: usize, right: usize| -> i32 {
            if fruits[right][0] <= start_pos {
                start_pos - fruits[left][0]
            } else if fruits[left][0] >= start_pos {
                fruits[right][0] - start_pos
            } else {
                std::cmp::min(
                    (start_pos - fruits[right][0]).abs(),
                    (start_pos - fruits[left][0]).abs(),
                ) + fruits[right][0]
                    - fruits[left][0]
            }
        };

        while right < n {
            sum += fruits[right][1];
            while left <= right && step(left, right) > k {
                sum -= fruits[left][1];
                left += 1;
            }
            ans = std::cmp::max(ans, sum);
            right += 1;
        }
        ans
    }
}
