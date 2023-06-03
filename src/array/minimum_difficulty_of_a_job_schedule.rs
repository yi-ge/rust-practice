// 工作计划的最低难度
// https://leetcode.cn/problems/minimum-difficulty-of-a-job-schedule
// INLINE  ../../images/array/minimum_difficulty_of_a_job_schedule.jpeg
// 官方题解：https://leetcode.cn/problems/minimum-difficulty-of-a-job-schedule/solution/gong-zuo-ji-hua-de-zui-di-nan-du-by-leet-dule/

pub struct Solution;

impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let n = job_difficulty.len();
        if n < d as usize {
            return -1; // 如果任务数小于要求的天数，则无法完成工作计划，返回 -1
        }
        let mut dp: Vec<i32> = vec![0; n];
        let mut max = 0;
        for j in 0..n {
            max = max.max(job_difficulty[j]); // 计算前 j 个任务中的最大难度
            dp[j] = max; // 将最大难度保存到 dp 数组中
        }
        for i in 1..d {
            let mut st: Vec<(usize, i32)> = Vec::new(); // 定义一个栈，用于保存当前天数下的最小值和对应的位置
            let mut ndp: Vec<i32> = vec![0; n]; // 定义一个新的 dp 数组，用于保存当前天数下的最小难度和前面天数的最小难度
            for j in i as usize..n {
                let mut mi = dp[j - 1]; // 初始化当前天数下的最小难度为前一个任务的最小难度
                while !st.is_empty() && job_difficulty[st.last().unwrap().0] < job_difficulty[j] {
                    // 如果栈不为空且栈顶位置的任务难度小于当前任务难度
                    mi = mi.min(st.last().unwrap().1); // 更新当前天数下的最小难度
                    st.pop(); // 弹出栈顶位置
                }
                if st.is_empty() {
                    ndp[j] = mi + job_difficulty[j]; // 如果栈为空，则当前天数下的最小难度为前面所有天数的最小难度加上当前任务的难度
                } else {
                    ndp[j] = ndp[st.last().unwrap().0].min(mi + job_difficulty[j]);
                    // 否则，当前天数下的最小难度为前面某个天数的最小难度加上当前任务的难度
                }
                st.push((j, mi)); // 将当前位置和最小难度入栈
            }
            dp = ndp; // 更新 dp 数组
        }
        dp[n - 1] // 返回完成所有任务所需的最小难度
    }
}
