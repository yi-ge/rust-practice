// 文件夹操作日志搜集器
// https://leetcode.cn/problems/crawler-log-folder
// INLINE  ../../images/stack/crawler_log_folder.jpeg

pub struct Solution;

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut res = 0;
        for i in 0..logs.len() {
            if logs[i] == "./" {
                continue;
            } else if logs[i] == "../" {
                if res > 0 {
                    res -= 1;
                }
            } else {
                res += 1;
            }
        }

        res
    }
}
