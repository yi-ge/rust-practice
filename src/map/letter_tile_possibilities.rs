// 活字印刷
// https://leetcode.cn/problems/letter-tile-possibilities
// INLINE  ../../images/map/letter_tile_possibilities.jpeg

pub struct Solution;

impl Solution {
    fn backtrack(result: &mut i32, chars: &mut Vec<char>, visited: &mut Vec<bool>) {
        for i in 0..chars.len() {
            if visited[i] {
                continue;
            }
            if i > 0 && chars[i] == chars[i - 1] && !visited[i - 1] {
                continue;
            }
            visited[i] = true;
            *result += 1;
            Self::backtrack(result, chars, visited);
            visited[i] = false;
        }
    }
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut result = 0;
        let mut chars = tiles.chars().collect::<Vec<char>>();
        chars.sort();
        let mut visited = vec![false; chars.len()];
        Self::backtrack(&mut result, &mut chars, &mut visited);
        result
    }
}
