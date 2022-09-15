pub fn insert_sort(mut nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {
        return vec![];
    }

    // 第一个元素视为已排列
    for i in 1..nums.len() {
        let curr = nums[i];

        let mut j = (i - 1) as i32;
        while j >= 0 {
            // 倒序循环
            if nums[j as usize] > curr {
                // 如果当前j位置元素比curr元素大，则将j位置元素右移
                nums[(j + 1) as usize] = nums[j as usize];
            } else {
                // 此时j+1所在的位置就是curr应该插入的位置
                break;
            }
            j -= 1;
        }

        nums[(j + 1) as usize] = curr;
    }

    nums
}
