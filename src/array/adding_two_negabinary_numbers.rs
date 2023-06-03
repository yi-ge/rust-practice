// 负二进制数相加
// https://leetcode.cn/problems/adding-two-negabinary-numbers
// INLINE  ../../images/array/adding_two_negabinary_numbers.jpeg

pub struct Solution;

impl Solution {
    pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut i = arr1.len() as i32 - 1; // arr1 数组的下标
        let mut j = arr2.len() as i32 - 1; // arr2 数组的下标
        let mut carry = 0; // 进位
        let mut ans = Vec::new(); // 存储结果的数组

        while i >= 0 || j >= 0 || carry != 0 { // 当两个数组都遍历完且没有进位时，循环结束
            let mut x = carry; // 当前位需要相加的值

            if i >= 0 {
                x += arr1[i as usize]; // 如果 arr1 数组还有值，则加上
            }
            if j >= 0 {
                x += arr2[j as usize]; // 如果 arr2 数组还有值，则加上
            }

            if x >= 2 { // 如果当前位相加结果大于等于 2
                ans.push(x - 2); // 将结果存入答案数组中
                carry = -1; // 进位为 -1
            } else if x >= 0 { // 如果当前位相加结果大于等于 0
                ans.push(x); // 将结果存入答案数组中
                carry = 0; // 进位为 0
            } else { // 如果当前位相加结果小于 0
                ans.push(1); // 将结果存入答案数组中
                carry = 1; // 进位为 1
            }

            i -= 1; // arr1 数组下标减 1
            j -= 1; // arr2 数组下标减 1
        }

        while ans.len() > 1 && *ans.last().unwrap() == 0 { // 去除答案数组末尾的 0
            ans.pop();
        }

        ans.reverse(); // 答案数组反转
        ans // 返回答案数组
    }
}