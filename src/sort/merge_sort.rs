// 实现归并排序
pub fn merge_sort(mut nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    if len <= 1 {
        return nums;
    }

    // 递归调用归并排序函数
    merge_sort_recursion(&mut nums, 0, len - 1);

    nums
}

// 归并函数，将两个有序数组合并成一个有序数组
fn merge(nums: &mut Vec<i32>, left: usize, middle: usize, right: usize) {
    // 初始化三个指针
    let mut i = left;
    let mut j = middle + 1;
    let mut k = left;

    // 创建临时数组
    let mut temp = vec![];

    // 将两个有序数组合并到临时数组中
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

    // 将临时数组中的元素赋值回原数组
    for i in 0..=(right - left) {
        nums[left + i] = temp[i];
    }
}

// 归并排序递归函数
fn merge_sort_recursion(nums: &mut Vec<i32>, left: usize, right: usize) {
    if left >= right {
        return;
    }

    // 计算数组中间位置
    let middle = (left + right) >> 1;
    // 递归调用归并排序函数
    merge_sort_recursion(nums, left, middle);
    merge_sort_recursion(nums, middle + 1, right);

    // 合并两个有序数组
    merge(nums, left, middle, right);
}
