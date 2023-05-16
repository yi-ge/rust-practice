// 工作计划的最低难度
// https://leetcode.cn/problems/minimum-difficulty-of-a-job-schedule
// INLINE  ../../images/array/minimum_difficulty_of_a_job_schedule.jpeg
// 官方题解：https://leetcode.cn/problems/minimum-difficulty-of-a-job-schedule/solution/gong-zuo-ji-hua-de-zui-di-nan-du-by-leet-dule/

pub struct Solution;

impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let n = job_difficulty.len();
        if n < d as usize {
            return -1;
        }
        let mut dp: Vec<i32> = vec![0; n];
        let mut max = 0;
        for j in 0..n {
            max = max.max(job_difficulty[j]);
            dp[j] = max;
        }
        for i in 1..d {
            let mut st: Vec<(usize, i32)> = Vec::new();
            let mut ndp: Vec<i32> = vec![0; n];
            for j in i as usize..n {
                let mut mi = dp[j - 1];
                while !st.is_empty() && job_difficulty[st.last().unwrap().0] < job_difficulty[j] {
                    mi = mi.min(st.last().unwrap().1);
                    st.pop();
                }
                if st.is_empty() {
                    ndp[j] = mi + job_difficulty[j];
                } else {
                    ndp[j] = ndp[st.last().unwrap().0].min(mi + job_difficulty[j]);
                }
                st.push((j, mi));
            }
            dp = ndp;
        }
        dp[n - 1]
    }
}
