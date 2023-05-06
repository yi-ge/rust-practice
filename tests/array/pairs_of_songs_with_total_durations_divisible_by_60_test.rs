use rust_practice::array::pairs_of_songs_with_total_durations_divisible_by_60::Solution;

#[test]
fn num_pairs_divisible_by60() {
    // 示例 1：
    // 输入：time = [30,20,150,100,40]
    // 输出：3
    // 解释：这三对的总持续时间可被 60 整除：
    // (time[0] = 30, time[2] = 150): 总持续时间 180
    // (time[1] = 20, time[3] = 100): 总持续时间 120
    // (time[1] = 20, time[4] = 40): 总持续时间 60
    let time = vec![30, 20, 150, 100, 40];
    assert_eq!(Solution::num_pairs_divisible_by60(time), 3);

    // 示例 2：
    // 输入：time = [60,60,60]
    // 输出：3
    // 解释：所有三对的总持续时间都是 120，可以被 60 整除。
    let time = vec![60, 60, 60];
    assert_eq!(Solution::num_pairs_divisible_by60(time), 3);
}
