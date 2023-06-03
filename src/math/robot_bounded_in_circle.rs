// 困于环中的机器人
// https://leetcode.cn/problems/robot-bounded-in-circle
// INLINE  ../../images/math/robot_bounded_in_circle.jpeg

pub struct Solution;

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut x = 0; // x坐标
        let mut y = 0; // y坐标
        let mut direction = 0; // 方向，0表示向北，1表示向东，2表示向南，3表示向西
        for c in instructions.chars() {
            match c {
                // 'G' => match direction {
                //     0 => y += 1,
                //     1 => x += 1,
                //     2 => y -= 1,
                //     3 => x -= 1,
                //     4..=i32::MAX => (),
                // },
                'G' => {
                    if direction == 0 {
                        // 若当前方向为向北，则y坐标加1
                        y += 1;
                    } else if direction == 1 {
                        // 若当前方向为向东，则x坐标加1
                        x += 1;
                    } else if direction == 2 {
                        // 若当前方向为向南，则y坐标减1
                        y -= 1;
                    } else if direction == 3 {
                        // 若当前方向为向西，则x坐标减1
                        x -= 1;
                    }
                }
                'L' => direction = (direction + 3) % 4, // 左转，方向数值加3取模4
                'R' => direction = (direction + 1) % 4, // 右转，方向数值加1取模4
                _ => (),                                // 其他字符不作处理
            }
        }
        (x == 0 && y == 0) || direction != 0 // 若返回原点或方向不为向北，则机器人必定在环内移动
    }
}
