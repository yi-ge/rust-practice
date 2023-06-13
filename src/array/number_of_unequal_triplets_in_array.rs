// 数组中不等三元组的数目
// https://leetcode.cn/problems/number-of-unequal-triplets-in-array
// INLINE  ../../images/array/number_of_unequal_triplets_in_array.jpeg

pub struct Solution;

impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let mut count = 0; // 初始化计数器
        let len = nums.len(); // 获取数组长度
        for i in 0..len {
            // 遍历数组
            for j in i + 1..len {
                // 遍历数组
                for k in j + 1..len {
                    // 遍历数组
                    if nums[i] != nums[j] && nums[j] != nums[k] && nums[i] != nums[k] {
                        // 判断是否三个数都不相等
                        count += 1; // 计数器加一
                    }
                }
            }
        }
        count // 返回计数器的值
    }
}
