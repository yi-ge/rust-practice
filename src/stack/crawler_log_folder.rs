// 文件夹操作日志搜集器
// https://leetcode.cn/problems/crawler-log-folder
// INLINE  ../../images/stack/crawler_log_folder.jpeg

pub struct Solution;

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut res = 0; // 记录当前所在文件夹的深度
        for i in 0..logs.len() {
            // 遍历操作日志
            if logs[i] == "./" {
                // 如果是当前文件夹，继续下一步操作
                continue;
            } else if logs[i] == "../" {
                // 如果是返回上一级文件夹，深度减一
                if res > 0 {
                    // 如果当前已经在根目录，深度不变
                    res -= 1;
                }
            } else {
                // 如果是进入下一级文件夹，深度加一
                res += 1;
            }
        }

        res // 返回最终的深度，即操作次数
    }
}
