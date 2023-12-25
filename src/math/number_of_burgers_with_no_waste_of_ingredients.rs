// 不浪费原料的汉堡制作方案
// https://leetcode.cn/problems/number-of-burgers-with-no-waste-of-ingredients
// INLINE  ../../images/math/number_of_burgers_with_no_waste_of_ingredients.jpeg

pub struct Solution;

impl Solution {
    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        let mut ret = Vec::new();
        if (tomato_slices >= 2 * cheese_slices)
            && (cheese_slices >= (tomato_slices - cheese_slices * 2) / 2)
            && ((tomato_slices - cheese_slices * 2) % 2 == 0)
        {
            ret.push((tomato_slices - cheese_slices * 2) / 2);
            ret.push(cheese_slices - (tomato_slices - cheese_slices * 2) / 2);
        }
        ret
    }
}
