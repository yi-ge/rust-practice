// 大样本统计
// https://leetcode.cn/problems/statistics-from-a-large-sample
// INLINE  ../../images/array/statistics_from_a_large_sample.jpeg

pub struct Solution;

impl Solution {
    pub fn sample_stats(count: Vec<i32>) -> Vec<f64> {
        let total = count.iter().sum::<i32>();
        #[allow(unused_assignments)]
        let mut mean = 0.0;
        let mut median = 0.0;
        let mut minnum = 256;
        let mut maxnum = 0;
        let mut mode = 0;

        let left = (total + 1) / 2;
        let right = (total + 2) / 2;
        let mut cnt = 0;
        let mut maxfreq = 0;
        let mut sum = 0;
        for (i, &c) in count.iter().enumerate() {
            sum += c as i64 * i as i64;
            if c > maxfreq {
                maxfreq = c;
                mode = i;
            }
            if c > 0 {
                if minnum == 256 {
                    minnum = i;
                }
                maxnum = i;
            }
            if cnt < right && cnt + c >= right {
                median += i as f64;
            }
            if cnt < left && cnt + c >= left {
                median += i as f64;
            }
            cnt += c;
        }
        mean = sum as f64 / total as f64;
        median = median / 2.0;
        vec![minnum as f64, maxnum as f64, mean, median, mode as f64]
    }
}
