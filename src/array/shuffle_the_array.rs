// 重新排列数组
// https://leetcode.cn/problems/shuffle-the-array/
// INLINE  ../../images/array/shuffle_the_array.jpeg

pub struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        // 使用迭代器对数组进行处理
        nums.iter()
            // zip方法将两个迭代器按位置配对，skip方法跳过前n个元素
            .zip(nums.iter().skip(n as usize))
            // flat_map方法将两个数作为数组输出
            .flat_map(|(&x, &y)| [x, y])
            // collect方法将迭代器转换为新的数组
            .collect()
    }
}
