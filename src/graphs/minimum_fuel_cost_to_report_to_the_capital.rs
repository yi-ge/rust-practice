// 到达首都的最少油耗
// https://leetcode.cn/problems/minimum-fuel-cost-to-report-to-the-capital
// INLINE  ../../images/graphs/minimum_fuel_cost_to_report_to_the_capital.jpeg

pub struct Solution;

impl Solution {
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        let n = roads.len() + 1;
        let mut graph = vec![vec![]; n];
        for road in &roads {
            let (src, dest): (usize, usize) = (road[0] as usize, road[1] as usize);
            graph[src].push(dest);
            graph[dest].push(src);
        }

        fn traverse(
            node: usize,
            parent: i32,
            graph: &Vec<Vec<usize>>,
            cost: &mut i64,
            seats: i32,
        ) -> i32 {
            let mut total = 1;
            for &neighbor in &graph[node] {
                if neighbor as i32 != parent {
                    let sub_total = traverse(neighbor, node as i32, graph, cost, seats);
                    *cost += ((sub_total + seats - 1) / seats) as i64;
                    total += sub_total;
                }
            }
            total
        }

        let mut cost = 0;
        traverse(0, -1, &graph, &mut cost, seats);
        cost
    }
}
