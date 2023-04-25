// 按身高排序
// https://leetcode.cn/problems/sort-the-people
// INLINE  ../../images/sort/sort_the_people.jpeg

pub struct Solution;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut result = vec![];
        let mut name_height = names
            .into_iter()
            .zip(heights.into_iter())
            .collect::<Vec<_>>();
        name_height.sort_by(|a, b| {
            if a.1 == b.1 {
                a.0.cmp(&b.0)
            } else {
                b.1.cmp(&a.1)
            }
        });
        for (name, _) in name_height {
            result.push(name);
        }
        result
    }
}
