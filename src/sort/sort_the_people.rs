// 按身高排序
// https://leetcode.cn/problems/sort-the-people
// INLINE  ../../images/sort/sort_the_people.jpeg

pub struct Solution;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        // 初始化结果数组
        let mut result = vec![];
        // 将姓名和身高组合成元组，并存入一个新的数组中
        let mut name_height = names
            .into_iter()
            .zip(heights.into_iter())
            .collect::<Vec<_>>();
        // 按照身高进行排序，如果身高相同则按姓名进行排序
        name_height.sort_by(|a, b| {
            if a.1 == b.1 {
                a.0.cmp(&b.0)
            } else {
                b.1.cmp(&a.1)
            }
        });
        // 将排好序的姓名存入结果数组中
        for (name, _) in name_height {
            result.push(name);
        }
        // 返回结果数组
        result
    }
}
