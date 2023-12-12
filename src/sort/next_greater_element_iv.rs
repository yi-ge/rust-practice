// 下一个更大元素 IV
// https://leetcode.cn/problems/next-greater-element-iv
// INLINE  ../../images/sort/next_greater_element_iv.jpeg

pub struct Solution;

impl Solution {
    pub fn second_greater_element(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![-1; nums.len()];
        let mut s = Vec::new();
        let mut t = Vec::new();
        for (i, &x) in nums.iter().enumerate() {
            while !t.is_empty() && nums[*t.last().unwrap()] < x {
                ans[t.pop().unwrap()] = x;
            }

            let mut j = s.len();

            while j > 0 && nums[s[j - 1]] < x {
                j -= 1;
            }

            t.extend(s.drain(j..));
            s.push(i);
        }
        ans
    }
}
