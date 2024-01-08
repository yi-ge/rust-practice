// 回旋镖的数量
// https://leetcode.cn/problems/number-of-boomerangs
// INLINE  ../../images/array/number_of_boomerangs.jpeg

pub struct Solution;

impl Solution {
    fn distance(p1: &Vec<i32>, p2: &Vec<i32>) -> i32 {
        (p1[0] - p2[0]).pow(2) + (p1[1] - p2[1]).pow(2)
    }
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        for i in 0..points.len() {
            let mut map = std::collections::HashMap::new();
            for j in 0..points.len() {
                if i != j {
                    let distance = Solution::distance(&points[i], &points[j]);
                    let count = map.entry(distance).or_insert(0);
                    *count += 1;
                }
            }
            for (_, v) in map {
                res += v * (v - 1);
            }
        }
        res
    }
}
