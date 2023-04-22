// 困于环中的机器人
// https://leetcode.cn/problems/robot-bounded-in-circle
// INLINE  ../../images/math/robot_bounded_in_circle.jpeg

pub struct Solution;

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut x = 0;
        let mut y = 0;
        let mut direction = 0;
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
                        y += 1;
                    } else if direction == 1 {
                        x += 1;
                    } else if direction == 2 {
                        y -= 1;
                    } else if direction == 3 {
                        x -= 1;
                    }
                }
                'L' => direction = (direction + 3) % 4,
                'R' => direction = (direction + 1) % 4,
                _ => (),
            }
        }
        (x == 0 && y == 0) || direction != 0
    }
}
