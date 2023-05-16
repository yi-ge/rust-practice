use rust_practice::array::minimum_difficulty_of_a_job_schedule::Solution;

#[test]
fn min_difficulty() {
    // 示例 1：
    // 输入：jobDifficulty = [6,5,4,3,2,1], d = 2
    // 输出：7
    // 解释：第一天，您可以完成前 5 项工作，总难度 = 6.
    // 第二天，您可以完成最后一项工作，总难度 = 1.
    // 计划表的难度 = 6 + 1 = 7
    let job_difficulty: Vec<i32> = vec![6, 5, 4, 3, 2, 1];
    let d = 2;
    assert_eq!(Solution::min_difficulty(job_difficulty, d), 7);

    // 示例 2：
    // 输入：jobDifficulty = [9,9,9], d = 4
    // 输出：-1
    // 解释：就算你每天完成一项工作，仍然有一天是空闲的，你无法制定一份能够满足既定工作时间的计划表。
    let job_difficulty: Vec<i32> = vec![9, 9, 9];
    let d = 4;
    assert_eq!(Solution::min_difficulty(job_difficulty, d), -1);

    // 示例 3：
    // 输入：jobDifficulty = [1,1,1], d = 3
    // 输出：3
    // 解释：工作计划为每天一项工作，总难度为 3 。
    let job_difficulty: Vec<i32> = vec![1, 1, 1];
    let d = 3;
    assert_eq!(Solution::min_difficulty(job_difficulty, d), 3);

    // 示例 4：
    // 输入：jobDifficulty = [7,1,7,1,7,1], d = 3
    // 输出：15
    let job_difficulty: Vec<i32> = vec![7, 1, 7, 1, 7, 1];
    let d = 3;
    assert_eq!(Solution::min_difficulty(job_difficulty, d), 15);

    // 示例 5：
    // 输入：jobDifficulty = [11,111,22,222,33,333,44,444], d = 6
    // 输出：843
    let job_difficulty: Vec<i32> = vec![11, 111, 22, 222, 33, 333, 44, 444];
    let d = 6;
    assert_eq!(Solution::min_difficulty(job_difficulty, d), 843);
}
