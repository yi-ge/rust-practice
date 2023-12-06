// 最小化旅行的价格总和
// https://leetcode.cn/problems/minimize-the-total-price-of-the-trips
// INLINE  ../../images/graphs/minimize_the_total_price_of_the_trips.jpeg
// 参考官方题解，dfs + dp

use std::cmp::min;

pub struct Solution;

impl Solution {
    pub fn dfs(
        node: usize,
        parent: usize,
        end: usize,
        next: &Vec<Vec<usize>>,
        count: &mut Vec<i32>,
    ) -> bool {
        if node == end {
            count[node] += 1;
            return true;
        }
        for &child in &next[node] {
            if child == parent {
                continue;
            }
            if Solution::dfs(child, node, end, next, count) {
                count[node] += 1;
                return true;
            }
        }
        false
    }

    pub fn dp(
        node: usize,
        parent: usize,
        next: &Vec<Vec<usize>>,
        price: &Vec<i32>,
        count: &Vec<i32>,
    ) -> (i32, i32) {
        let mut res: (i32, i32) = (price[node] * count[node], price[node] * count[node] / 2);
        for &child in &next[node] {
            if child == parent {
                continue;
            }
            let (x, y) = Solution::dp(child, node, next, price, count);
            res.0 += min(x, y);
            res.1 += x;
        }
        res
    }

    pub fn minimum_total_price(
        n: i32,
        edges: Vec<Vec<i32>>,
        price: Vec<i32>,
        trips: Vec<Vec<i32>>,
    ) -> i32 {
        let n = n as usize;
        let mut next: Vec<Vec<usize>> = vec![vec![]; n];
        for edge in &edges {
            next[edge[0] as usize].push(edge[1] as usize);
            next[edge[1] as usize].push(edge[0] as usize);
        }
        let mut count: Vec<i32> = vec![0; n];
        for trip in &trips {
            Solution::dfs(
                trip[0] as usize,
                usize::MAX,
                trip[1] as usize,
                &next,
                &mut count,
            );
        }
        let (x, y) = Solution::dp(0, usize::MAX, &next, &price, &count);
        min(x, y)
    }
}
