// 总持续时间可被 60 整除的歌曲
// https://leetcode.cn/problems/pairs-of-songs-with-total-durations-divisible-by-60
// INLINE  ../../images/array/pairs_of_songs_with_total_durations_divisible_by_60.jpeg

pub struct Solution;

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut remainders = [0; 60];
        for t in time {
            let remainder = t % 60;
            let complement = (60 - remainder) % 60;
            count += remainders[complement as usize];
            remainders[remainder as usize] += 1;
        }
        count
    }
}
