use rust_practice::math::number_of_burgers_with_no_waste_of_ingredients::Solution;

#[test]
fn num_of_burgers() {
    // 示例 1：
    // 输入：tomatoSlices = 16, cheeseSlices = 7
    // 输出：[1,6]
    // 解释：制作 1 个巨无霸汉堡和 6 个小皇堡需要 4*1 + 2*6 = 16 片番茄和 1 + 6 = 7 片奶酪。不会剩下原料。
    let tomato_slices = 16;
    let cheese_slices = 7;
    let result = Solution::num_of_burgers(tomato_slices, cheese_slices);
    assert_eq!(result, vec![1, 6]);

    // 示例 2：
    // 输入：tomatoSlices = 17, cheeseSlices = 4
    // 输出：[]
    // 解释：只制作小皇堡和巨无霸汉堡无法用光全部原料。
    let tomato_slices = 17;
    let cheese_slices = 4;
    let result = Solution::num_of_burgers(tomato_slices, cheese_slices);
    assert_eq!(result, vec![]);

    // 示例 3：
    // 输入：tomatoSlices = 4, cheeseSlices = 17
    // 输出：[]
    // 解释：制作 1 个巨无霸汉堡会剩下 16 片奶酪，制作 2 个小皇堡会剩下 15 片奶酪。
    let tomato_slices = 4;
    let cheese_slices = 17;
    let result = Solution::num_of_burgers(tomato_slices, cheese_slices);
    assert_eq!(result, vec![]);

    // 示例 4：
    // 输入：tomatoSlices = 0, cheeseSlices = 0
    // 输出：[0,0]
    let tomato_slices = 0;
    let cheese_slices = 0;
    let result = Solution::num_of_burgers(tomato_slices, cheese_slices);
    assert_eq!(result, vec![0, 0]);

    // 示例 5：
    // 输入：tomatoSlices = 2, cheeseSlices = 1
    // 输出：[0,1]
    let tomato_slices = 2;
    let cheese_slices = 1;
    let result = Solution::num_of_burgers(tomato_slices, cheese_slices);
    assert_eq!(result, vec![0, 1]);
}
