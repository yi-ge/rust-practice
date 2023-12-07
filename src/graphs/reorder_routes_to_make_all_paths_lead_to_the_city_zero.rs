// 重新规划路线
// https://leetcode.cn/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero
// INLINE  ../../images/graphs/reorder_routes_to_make_all_paths_lead_to_the_city_zero.jpeg

pub struct Solution;

impl Solution {
    fn dfs(node: i32, visited: &mut Vec<bool>, res: &mut i32, graph: &Vec<Vec<(i32, bool)>>) {
        visited[node as usize] = true;
        for (next, is_out) in &graph[node as usize] {
            if !visited[*next as usize] {
                if *is_out {
                    *res += 1;
                }
                Self::dfs(*next, visited, res, graph);
            }
        }
    }
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut visited = vec![false; n as usize];
        let mut graph = vec![vec![]; n as usize];
        for connection in connections {
            graph[connection[0] as usize].push((connection[1], true));
            graph[connection[1] as usize].push((connection[0], false));
        }
        Self::dfs(0, &mut visited, &mut res, &graph);
        res
    }
}
