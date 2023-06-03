// 判断两个事件是否存在冲突
// https://leetcode.cn/problems/determine-if-two-events-have-conflict
// INLINE  ../../images/array/determine_if_two_events_have_conflict.jpeg
pub struct Solution;

impl Solution {
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        // 如果event1的开始时间晚于event2的结束时间，或者event1的结束时间早于event2的开始时间，则两个事件不存在冲突
        !(event1[0] > event2[1] || event1[1] < event2[0])
    }
}
