// * 堆化（heapify）：把一个无序数组整理成满足堆特性的堆数组

// 从下往上的堆化
// 从索引为1的元素开始，依次与父结点值比较大小。
// 不满足子结点值小于父结点值则交换。
pub fn build_heap_down_up(nums: &mut Vec<i32>) {
    for i in 1..nums.len() {
        heapify_down_up(nums, i);
    }
}

fn heapify_down_up(nums: &mut Vec<i32>, index: usize) {
    let mut index = index;
    let mut parent_index = (index - 1) >> 1; // 父结点
    while nums[index] > nums[parent_index] {
        // 如果当前结点大于父结点
        nums.swap(index, parent_index); // 交换结点
        index = parent_index;
        if index == 0 {
            break;
        }
        parent_index = (index - 1) >> 1;
    }
}

// 从下往上的堆化
// 对索引 n/2 开始到 0 的元素，依次取元素的值与其子结点的值比较大小。
// 不满足父结点值大于子结点值则交换。
pub fn build_heap_up_down(nums: &mut Vec<i32>) {
    let len = nums.len();
    for i in (0..len / 2).rev() {
        heapify_up_down(nums, i, len);
    }
}

fn heapify_up_down(nums: &mut Vec<i32>, index: usize, len: usize) {
    let mut index = index;
    loop {
        let mut max_pos = index; // 假设当前结点是最大值的结点

        // 判断当前结点值是否小于左子结点，如果是则将左子结点设置为最大值结点
        if 2 * index + 1 < len && nums[index] < nums[2 * index + 1] {
            max_pos = 2 * index + 1;
        }

        // 判断假设的最大值结点是否小于右子结点，如果是则将有字及诶单设置为最大值结点
        if 2 * index + 2 < len && nums[max_pos] < nums[2 * index + 2] {
            max_pos = 2 * index + 2;
        }

        if max_pos == index {
            break;
        }

        nums.swap(index, max_pos); // 交换结点

        index = max_pos;
    }
}
