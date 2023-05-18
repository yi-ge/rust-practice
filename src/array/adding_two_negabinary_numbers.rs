// 负二进制数相加
// https://leetcode.cn/problems/adding-two-negabinary-numbers
// INLINE  ../../images/array/adding_two_negabinary_numbers.jpeg

pub struct Solution;

impl Solution {
    pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut i = arr1.len() as i32 - 1;
        let mut j = arr2.len() as i32 - 1;
        let mut carry = 0;
        let mut ans = Vec::new();

        while i >= 0 || j >= 0 || carry != 0 {
            let mut x = carry;

            if i >= 0 {
                x += arr1[i as usize];
            }
            if j >= 0 {
                x += arr2[j as usize];
            }

            if x >= 2 {
                ans.push(x - 2);
                carry = -1;
            } else if x >= 0 {
                ans.push(x);
                carry = 0;
            } else {
                ans.push(1);
                carry = 1;
            }

            i -= 1;
            j -= 1;
        }

        while ans.len() > 1 && *ans.last().unwrap() == 0 {
            ans.pop();
        }

        ans.reverse();
        ans
    }
}
