// 最小的必要团队
// https://leetcode.cn/problems/smallest-sufficient-team
// INLINE  ../../images/math/smallest_sufficient_team.jpeg
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        let n = req_skills.len();
        let m = people.len();
        let mut skill_index: HashMap<String, usize> = HashMap::new();
        for i in 0..n {
            skill_index.insert(req_skills[i].clone(), i);
        }
        let mut dp = vec![m; 1 << n];
        dp[0] = 0;
        let mut prev_skill = vec![0; 1 << n];
        let mut prev_people = vec![0; 1 << n];
        for i in 0..m {
            let mut cur_skill = 0;
            for skill in &people[i] {
                cur_skill |= 1 << skill_index[skill];
            }
            for prev in 0..(1 << n) {
                let comb = prev | cur_skill;
                if dp[comb] > dp[prev] + 1 {
                    dp[comb] = dp[prev] + 1;
                    prev_skill[comb] = prev;
                    prev_people[comb] = i;
                }
            }
        }
        let mut res = Vec::new();
        let mut i = (1 << n) - 1;
        while i > 0 {
            res.push(prev_people[i] as i32);
            i = prev_skill[i];
        }
        res
    }
}
