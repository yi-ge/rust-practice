// 参加考试的最大学生数
// https://leetcode.cn/problems/maximum-students-taking-exam
// INLINE  ../../images/math/maximum_students_taking_exam.jpeg

pub struct Solution;

impl Solution {
    pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
        fn dfs(
            seat: usize,
            i: usize,
            f: &mut Vec<Vec<i32>>,
            ss: &Vec<usize>,
            m: usize,
            n: usize,
        ) -> i32 {
            if f[seat][i] != -1 {
                return f[seat][i];
            }
            let mut ans = 0;
            for mask in 0..1 << n {
                if (seat | mask) != seat || (mask & (mask << 1)) != 0 {
                    continue;
                }
                let cnt = mask.count_ones();
                if i == m - 1 {
                    ans = std::cmp::max(ans, cnt as i32);
                } else {
                    let mut nxt = ss[i + 1];
                    nxt &= !(mask >> 1);
                    nxt &= !(mask << 1);
                    ans = std::cmp::max(ans, cnt as i32 + dfs(nxt, i + 1, f, ss, m, n));
                }
            }
            f[seat][i] = ans;
            ans
        }

        let m = seats.len();
        let n = seats[0].len();
        let mut ss = vec![0; m];
        let mut f = vec![vec![-1; m]; 1 << n];

        for i in 0..m {
            for j in 0..n {
                if seats[i][j] == '.' {
                    ss[i] |= 1 << j;
                }
            }
        }

        dfs(ss[0], 0, &mut f, &ss, m, n)
    }
}
