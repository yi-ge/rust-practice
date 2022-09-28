pub fn merge_sort(mut nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    if len <= 1 {
        return nums;
    }

    merge_sort_recursion(&mut nums, 0, len - 1);

    nums
}

fn merge(nums: &mut Vec<i32>, left: usize, middle: usize, right: usize) {
    let mut i = left;
    let mut j = middle + 1;
    let mut k = left;

    let mut temp = vec![];

    while k <= right {
        if i > middle {
            temp.push(nums[j]);
            j += 1;
            k += 1;
        } else if j > right {
            temp.push(nums[i]);
            i += 1;
            k += 1;
        } else if nums[i] < nums[j] {
            temp.push(nums[i]);
            i += 1;
            k += 1;
        } else {
            temp.push(nums[j]);
            j += 1;
            k += 1;
        }
    }

    for i in 0..=(right - left) {
        nums[left + i] = temp[i];
    }
}

fn merge_sort_recursion(nums: &mut Vec<i32>, left: usize, right: usize) {
    if left >= right {
        return;
    }

    let middle = (left + right) >> 1;
    merge_sort_recursion(nums, left, middle);
    merge_sort_recursion(nums, middle + 1, right);

    merge(nums, left, middle, right);
}
