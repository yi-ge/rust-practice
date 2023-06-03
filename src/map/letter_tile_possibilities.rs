// 活字印刷
// https://leetcode.cn/problems/letter-tile-possibilities
// INLINE  ../../images/map/letter_tile_possibilities.jpeg

pub struct Solution;

impl Solution {
    // 回溯函数，result表示结果计数器，chars表示可用字符集合，visited表示字符是否已经被使用过
    fn backtrack(result: &mut i32, chars: &mut Vec<char>, visited: &mut Vec<bool>) {
        // 遍历每个字符
        for i in 0..chars.len() {
            // 如果该字符已经被使用过，则跳过该字符
            if visited[i] {
                continue;
            }
            // 如果该字符与前一个字符相同且前一个字符未被使用，则跳过该字符
            if i > 0 && chars[i] == chars[i - 1] && !visited[i - 1] {
                continue;
            }
            // 将该字符标记为已使用
            visited[i] = true;
            // 结果计数器加1
            *result += 1;
            // 进行下一层回溯
            Self::backtrack(result, chars, visited);
            // 将该字符标记为未使用，以便回溯到上一层继续尝试其他字符的使用情况
            visited[i] = false;
        }
    }
    // 计算可以组成的不同的排列数量
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut result = 0;
        let mut chars = tiles.chars().collect::<Vec<char>>();
        chars.sort(); // 对字符进行排序，以便在回溯过程中去重
        let mut visited = vec![false; chars.len()]; // 初始化visited数组
        Self::backtrack(&mut result, &mut chars, &mut visited); // 进行回溯
        result // 返回结果
    }
}
