use rust_practice::math::robot_bounded_in_circle::Solution;

#[test]
fn is_robot_bounded() {
    // 示例 1：
    // 输入：instructions = "GGLLGG"
    // 输出：true
    // 解释：机器人最初在(0,0)处，面向北方。
    // “G”:移动一步。位置:(0,1)方向:北。
    // “G”:移动一步。位置:(0,2).方向:北。
    // “L”:逆时针旋转90度。位置:(0,2).方向:西。
    // “L”:逆时针旋转90度。位置:(0,2)方向:南。
    // “G”:移动一步。位置:(0,1)方向:南。
    // “G”:移动一步。位置:(0,0)方向:南。
    // 重复指令，机器人进入循环:(0,0)——>(0,1)——>(0,2)——>(0,1)——>(0,0)。
    // 在此基础上，我们返回true。
    let instructions1 = "GGLLGG";
    assert_eq!(Solution::is_robot_bounded(instructions1.to_string()), true);

    // 示例 2：
    // 输入：instructions = "GG"
    // 输出：false
    // 解释：机器人最初在(0,0)处，面向北方。
    // “G”:移动一步。位置:(0,1)方向:北。
    // “G”:移动一步。位置:(0,2).方向:北。
    // 重复这些指示，继续朝北前进，不会进入循环。
    // 在此基础上，返回false。
    let instructions2 = "GG";
    assert_eq!(Solution::is_robot_bounded(instructions2.to_string()), false);

    // 示例 3：
    // 输入：instructions = "GL"
    // 输出：true
    // 解释：机器人最初在(0,0)处，面向北方。
    // “G”:移动一步。位置:(0,1)方向:北。
    // “L”:逆时针旋转90度。位置:(0,1).方向:西。
    // “G”:移动一步。位置:(- 1,1)方向:西。
    // “L”:逆时针旋转90度。位置:(- 1,1)方向:南。
    // “G”:移动一步。位置:(- 1,0)方向:南。
    // “L”:逆时针旋转90度。位置:(- 1,0)方向:东方。
    // “G”:移动一步。位置:(0,0)方向:东方。
    // “L”:逆时针旋转90度。位置:(0,0)方向:北。
    // 重复指令，机器人进入循环:(0,0)——>(0,1)——>(- 1,1)——>(- 1,0)——>(0,0)。
    // 在此基础上，我们返回true。
    let instructions3 = "GL";
    assert_eq!(Solution::is_robot_bounded(instructions3.to_string()), true);

    // 示例 4：测试字符“R”分支
    let instructions4 = "GRGR";
    assert_eq!(Solution::is_robot_bounded(instructions4.to_string()), true);

    // 示例 5：测试字符“L”和“R”组合的分支
    let instructions5 = "GLGR";
    assert_eq!(Solution::is_robot_bounded(instructions5.to_string()), false);

    // 示例 6：测试空字符串
    let instructions6 = "";
    assert_eq!(Solution::is_robot_bounded(instructions6.to_string()), true);

    // 示例 7：测试无效字符
    let instructions7 = "ABCD";
    assert_eq!(Solution::is_robot_bounded(instructions7.to_string()), true);
}
