// 总持续时间可被 60 整除的歌曲
// https://leetcode.cn/problems/pairs-of-songs-with-total-durations-divisible-by-60
// INLINE  ../../images/array/pairs_of_songs_with_total_durations_divisible_by_60.jpeg

pub struct Solution;

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut count = 0; // 统计总数
        let mut remainders = [0; 60]; // 余数数组，用于存储每个余数出现的次数
        for t in time {
            let remainder = t % 60; // 计算当前歌曲的余数
            let complement = (60 - remainder) % 60; // 计算与当前歌曲余数相加可以得到 60 的余数
            count += remainders[complement as usize]; // 将与当前歌曲余数相加可以得到 60 的余数出现的次数加到统计总数中
            remainders[remainder as usize] += 1; // 将当前歌曲余数出现的次数加 1
        }
        count // 返回统计总数
    }
}
