# Rust练习

[![license](https://img.shields.io/github/license/yi-ge/rust-practice.svg?style=flat-square)](https://github.com/yi-ge/rust-practice/blob/master/LICENSE)
[![GitHub Actions](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fyi-ge%2Frust-practice%2Fbadge%3Fref%3Dmain&style=flat-square)](https://actions-badge.atrox.dev/yi-ge/rust-practice/goto?ref=main)
[![Test Results](https://gist.github.com/yi-ge/00fdcacb47689d14b8e9fdf7fb0f7288/raw/badge.svg)](https://github.com/yi-ge/rust-practice)
[![Coveralls github](https://img.shields.io/coveralls/github/yi-ge/rust-practice?style=flat-square)](https://coveralls.io/github/yi-ge/rust-practice)
[![GitHub last commit](https://img.shields.io/github/last-commit/yi-ge/rust-practice.svg?style=flat-square)](https://github.com/yi-ge/rust-practice)
[![Gitpod Ready-to-Code](https://img.shields.io/badge/Gitpod-Ready--to--Code-blue?logo=gitpod&style=flat-square)](https://gitpod.io/#https://github.com/yi-ge/rust-practice)

Rust 基础算法、数据结构练习，包含 LeetCode 或其它算法练习记录。

此为个人练习仓库，代码中对重要思想进行了注释，会尽量补充解题思路。

每一道题都对应写有测试用例，但可能不够完整。如果您发现错误，欢迎给我留言，谢谢！

安装以下测试环境后，运行`yarn start`可以自动从LeetCode获取代码函数和用例说明。保存文件后将自动同步到浏览器。

特别说明：题目截图仅为了方便在代码编辑器中直接预览从而优化编码体验，题目以LeetCode官方页面为准，题目著作权及其他权利以LeetCode官方说明为准或属于LeetCode。请大家尊重版权，共同维护良好网络环境。

## 测试环境

安装最新版[Rust](https://www.rust-lang.org/)、[Node.js](https://nodejs.org)和[Python3](https://www.python.org/)。安装完成后执行`yarn`安装依赖。

### 非CI环境的Linux x86平台下

单元覆盖率测试依赖：`cargo install cargo-watch cargo-tarpaulin`以及`VSCode`插件[Coverage Gutters](https://marketplace.visualstudio.com/items?itemName=ryanluker.vscode-coverage-gutters)。

执行命令`cargo watch -x 'tarpaulin --ignore-tests --out Lcov' -i lcov.info`，可以在`VSCode`中查看覆盖情况。

可[参考](https://dev.to/marcoieni/2-videos-about-rust-code-coverage-in-vscode-38kf)进行覆盖率测试。

### 非CI环境的MacOS x86/ARM平台下

需要安装`nightly`版本的构建工具用于单元覆盖率测试（仅用于单元覆盖率测试，否则可以使用`stable`版本）。

如果已经通过`brew`安装了rust，可以先`brew uninstall rust`再执行`brew install rustup-init`安装`rustup-init`。执行`rustup-init`就有了`rustup`。后续通过`rustup`管理`rust`版本（如非覆盖率测试必要，仍然建议使用`brew install rust`安装稳定的`rust`版本）。

设置为`nightly`版本：

```bash
rustup default nightly
```

安装覆盖率测试相关依赖：

```bash
rustup component add llvm-tools-preview
cargo install grcov
```

执行本地覆盖率测试（在HTML中查看）：

```bash
cargo xtask coverage --dev
```

安装`VSCode`插件[Coverage Gutters](https://marketplace.visualstudio.com/items?itemName=ryanluker.vscode-coverage-gutters)。执行本地覆盖率测试（`VSCode`中查看）：

```bash
cargo xtask coverage
```

右键：`Coverage Gutters: Display Coverage`。

## 基础排序算法

- [冒泡排序](src/sort/bubble_sort.rs)
- [插入排序](src/sort/insert_sort.rs)
- [选择排序](src/sort/select_sort.rs)
- [堆排序](src/sort/heap_sort.rs)
- [快速排序](src/sort/quick_sort.rs)
- [归并排序](src/sort/merge_sort.rs)

## 基础数据结构

### Rust标准库中的数据结构

Rust标准库`std::collections`提供了4种通用容器类型，包含一下8种数据结构。

| 类型     | 容器        | 描述                     |
| -------- | :---------- | ------------------------ |
| 线性序列 | Vec\<T>      | 连续存储的可变长数组     |
| 线性序列 | VecDeque\<T> | 连续存储的可变长双端队列 |
| 线性序列 | LinkedList\<T> | 非连续存储的双向链表 |
| 键 - 值对 | HashMap\<K, V> | 基于哈希表的无序键 - 值对 |
| 键 - 值对 | BTreeMap\<K, V> | 基于B树的有序键 - 值对，按 Key 排序 |
| 集合 | HashSet\<T> | 基于哈希表的无序集合 |
| 集合 | BTreeSet\<T> | 基于B树的有序集合 |
| 优先队列 | BinaryHeap\<T> | 基于二叉堆的优先队列 |

### 通过Rust实现的数据结构及其常见操作

- [链表结点](src/libs/list_node.rs)
- [链表操作](src/libs/list.rs)
- [树结点](src/libs/tree_node.rs)
- [二叉树的前序遍历](src/tree/binary_tree_preorder_traversal.rs)
- [二叉树的中序遍历](src/tree/binary_tree_inorder_traversal.rs)
- [二叉树的后序遍历](src/tree/binary_tree_postorder_traversal.rs)
- [二叉树的层序遍历](src/tree/binary_tree_level_order_traversal.rs)
- [堆化、插入及删除堆元素](src/libs/heap.rs)

## 算法题

### 字符串

- [统计重复个数](src/string/count_the_repetitions.rs)  [字符串, 动态规划]

  - LeetCode 466. 统计重复个数 <https://leetcode.cn/problems/count-the-repetitions>

- [字典序最小回文串](src/string/lexicographically_smallest_palindrome.rs)  [贪心, 双指针, 字符串]

  - LeetCode 2697. 字典序最小回文串 <https://leetcode.cn/problems/lexicographically-smallest-palindrome>

- [子串能表示从 1 到 N 数字的二进制串](src/string/binary_string_with_substrings_representing_1_to_n.rs)  [字符串]

  - LeetCode 1016. 子串能表示从 1 到 N 数字的二进制串 <https://leetcode.cn/problems/binary-string-with-substrings-representing-1-to-n>

- [有效时间的数目](src/string/number_of_valid_clock_times.rs)  [字符串, 枚举]

  - LeetCode 2437. 有效时间的数目 <https://leetcode.cn/problems/number-of-valid-clock-times>

- [数青蛙](src/string/minimum_number_of_frogs_croaking.rs)  [字符串, 计数]

  - LeetCode 1419. 数青蛙 <https://leetcode.cn/problems/minimum-number-of-frogs-croaking>

- [按字典序排在最后的子串](src/string/last_substring_in_lexicographical_order.rs)  [双指针, 字符串]

  - LeetCode 1163. 按字典序排在最后的子串 <https://leetcode.cn/problems/last-substring-in-lexicographical-order>

- [段式回文](src/string/longest_chunked_palindrome_decomposition.rs)  [贪心, 双指针, 字符串, 动态规划, 哈希函数, 滚动哈希]

  - LeetCode 1147. 段式回文 <https://leetcode.cn/problems/longest-chunked-palindrome-decomposition>

- [隐藏个人信息](src/string/masking_personal_information.rs)  [字符串]

  - LeetCode 831. 隐藏个人信息 <https://leetcode.cn/problems/masking-personal-information>

- [最短公共超序列](src/string/shortest_common_supersequence.rs)  [字符串, 动态规划]

  - LeetCode 1092. 最短公共超序列 <https://leetcode.cn/problems/shortest-common-supersequence>

- [检查二进制字符串字段](src/string/check_if_binary_string_has_at_most_one_segment_of_ones.rs)  [字符串]

  - LeetCode 1784. 检查二进制字符串字段 <https://leetcode.cn/problems/check-if-binary-string-has-at-most-one-segment-of-ones>

- [重新格式化电话号码](src/string/reformat_phone_number.rs)  [字符串]

  - LeetCode 1694. 重新格式化电话号码 <https://leetcode.cn/problems/reformat-phone-number>

- [字符串轮转](src/string/string_rotation_lcci.rs)  [字符串, 字符串匹配]

  - LeetCode 面试题 01.09. 字符串轮转 <https://leetcode.cn/problems/string-rotation-lcci>

- [检查单词是否为句中其他单词的前缀](src/string/check_if_a_word_occurs_as_a_prefix_of_any_word_in_a_sentence.rs)  [字符串, 字符串匹配]

  - LeetCode 1455. 检查单词是否为句中其他单词的前缀 <https://leetcode.cn/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence>

- [重新格式化字符串](src/string/reformat_the_string.rs)  [字符串]

  - LeetCode 1417. 重新格式化字符串 <https://leetcode.cn/problems/reformat-the-string>

### 数组/队列/集合/映射

- [回旋镖的数量](src/array/number_of_boomerangs.rs)  [数组, 哈希表, 数学]

  - LeetCode 447. 回旋镖的数量 <https://leetcode.cn/problems/number-of-boomerangs>

- [经营摩天轮的最大利润](src/array/maximum_profit_of_operating_a_centennial_wheel.rs)  [数组, 模拟]

  - LeetCode 1599. 经营摩天轮的最大利润 <https://leetcode.cn/problems/maximum-profit-of-operating-a-centennial-wheel>

- [使用最小花费爬楼梯](src/array/min_cost_climbing_stairs.rs)  [数组, 动态规划]

  - LeetCode 746. 使用最小花费爬楼梯 <https://leetcode.cn/problems/min-cost-climbing-stairs>

- [用邮票贴满网格图](src/array/stamping_the_grid.rs)  [贪心, 数组, 矩阵, 前缀和]

  - LeetCode 2132. 用邮票贴满网格图 <https://leetcode.cn/problems/stamping-the-grid>

- [打家劫舍](src/array/house_robber.rs)  [数组, 动态规划]

  - LeetCode 198. 打家劫舍 <https://leetcode.cn/problems/house-robber>

- [二进制字符串前缀一致的次数](src/array/number_of_times_binary_string_is_prefix_aligned.rs)  [数组]

  - LeetCode 1375. 二进制字符串前缀一致的次数 <https://leetcode.cn/problems/number-of-times-binary-string-is-prefix-aligned>

- [数组中不等三元组的数目](src/array/number_of_unequal_triplets_in_array.rs)  [数组, 哈希表]

  - LeetCode 2475. 数组中不等三元组的数目 <https://leetcode.cn/problems/number-of-unequal-triplets-in-array>

- [相等行列对](src/array/equal_row_and_column_pairs.rs)  [数组, 哈希表, 矩阵, 模拟]

  - LeetCode 2352. 相等行列对 <https://leetcode.cn/problems/equal-row-and-column-pairs>

- [对数组执行操作](src/array/apply_operations_to_an_array.rs)  [数组, 模拟]

  - LeetCode 2460. 对数组执行操作 <https://leetcode.cn/problems/apply-operations-to-an-array>

- [统计范围内的元音字符串数](src/array/count_vowel_strings_in_ranges.rs)  [数组, 字符串, 前缀和]

  - LeetCode 2559. 统计范围内的元音字符串数 <https://leetcode.cn/problems/count-vowel-strings-in-ranges>

- [可被三整除的偶数的平均值](src/array/average_value_of_even_numbers_that_are_divisible_by_three.rs)  [数组, 数学]

  - LeetCode 2455. 可被三整除的偶数的平均值 <https://leetcode.cn/problems/average-value-of-even-numbers-that-are-divisible-by-three>

- [有序矩阵中的第 k 个最小数组和](src/array/find_the_kth_smallest_sum_of_a_matrix_with_sorted_rows.rs)  [数组, 二分查找, 矩阵, 堆（优先队列）]

  - LeetCode 1439. 有序矩阵中的第 k 个最小数组和 <https://leetcode.cn/problems/find-the-kth-smallest-sum-of-a-matrix-with-sorted-rows>

- [大样本统计](src/array/statistics_from_a_large_sample.rs)  [数组, 数学, 概率与统计]

  - LeetCode 1093. 大样本统计 <https://leetcode.cn/problems/statistics-from-a-large-sample>

- [差值数组不同的字符串](src/array/odd_string_difference.rs)  [数组, 哈希表, 字符串]

  - LeetCode 2451. 差值数组不同的字符串 <https://leetcode.cn/problems/odd-string-difference>

- [蓄水](src/array/store_water.rs)  [贪心, 数组, 堆（优先队列）]

  - LeetCode LCP 33. 蓄水 <https://leetcode.cn/problems/o8SXZn>

- [负二进制数相加](src/array/adding_two_negabinary_numbers.rs)  [数组, 数学]

  - LeetCode 1073. 负二进制数相加 <https://leetcode.cn/problems/adding-two-negabinary-numbers>

- [判断两个事件是否存在冲突](src/array/determine_if_two_events_have_conflict.rs)  [数组, 字符串]

  - LeetCode 2446. 判断两个事件是否存在冲突 <https://leetcode.cn/problems/determine-if-two-events-have-conflict>

- [工作计划的最低难度](src/array/minimum_difficulty_of_a_job_schedule.rs)  [数组, 动态规划]

  - LeetCode 1335. 工作计划的最低难度 <https://leetcode.cn/problems/minimum-difficulty-of-a-job-schedule>

- [按列翻转得到最大值等行数](src/array/flip_columns_for_maximum_number_of_equal_rows.rs)  [数组, 哈希表, 矩阵]

  - LeetCode 1072. 按列翻转得到最大值等行数 <https://leetcode.cn/problems/flip-columns-for-maximum-number-of-equal-rows>

- [翻转子数组得到最大的数组值](src/array/reverse_subarray_to_maximize_array_value.rs)  [贪心, 数组, 数学]

  - LeetCode 1330. 翻转子数组得到最大的数组值 <https://leetcode.cn/problems/reverse-subarray-to-maximize-array-value>

- [总持续时间可被 60 整除的歌曲](src/array/pairs_of_songs_with_total_durations_divisible_by_60.rs)  [数组, 哈希表, 计数]

  - LeetCode 1010. 总持续时间可被 60 整除的歌曲 <https://leetcode.cn/problems/pairs-of-songs-with-total-durations-divisible-by-60>

- [处理用时最长的那个任务的员工](src/array/the_employee_that_worked_on_the_longest_task.rs)  [数组]

  - LeetCode 2432. 处理用时最长的那个任务的员工 <https://leetcode.cn/problems/the-employee-that-worked-on-the-longest-task>

- [摘水果](src/array/maximum_fruits_harvested_after_at_most_k_steps.rs)  [数组, 二分查找, 前缀和, 滑动窗口]

  - LeetCode 2106. 摘水果 <https://leetcode.cn/problems/maximum-fruits-harvested-after-at-most-k-steps>

- [最长字符串链](src/array/longest_string_chain.rs)  [数组, 哈希表, 双指针, 字符串, 动态规划]

  - LeetCode 1048. 最长字符串链 <https://leetcode.cn/problems/longest-string-chain>

- [两个非重叠子数组的最大和](src/array/maximum_sum_of_two_non_overlapping_subarrays.rs)  [数组, 动态规划, 滑动窗口]

  - LeetCode 1031. 两个非重叠子数组的最大和 <https://leetcode.cn/problems/maximum-sum-of-two-non-overlapping-subarrays>

- [填充书架](src/array/filling_bookcase_shelves.rs)  [数组, 动态规划]

  - LeetCode 1105. 填充书架 <https://leetcode.cn/problems/filling-bookcase-shelves>

- [最长等差数列](src/array/longest_arithmetic_subsequence.rs)  [数组, 哈希表, 二分查找, 动态规划]

  - LeetCode 1027. 最长等差数列 <https://leetcode.cn/problems/longest-arithmetic-subsequence>

- [分隔数组以得到最大和](src/array/partition_array_for_maximum_sum.rs)  [数组, 动态规划]

  - LeetCode 1043. 分隔数组以得到最大和 <https://leetcode.cn/problems/partition-array-for-maximum-sum>

- [子数组中占绝大多数的元素](src/array/online_majority_element_in_subarray.rs)  [设计, 树状数组, 线段树, 数组, 二分查找]

  - LeetCode 1157. 子数组中占绝大多数的元素 <https://leetcode.cn/problems/online-majority-element-in-subarray>

- [出现最频繁的偶数元素](src/array/most_frequent_even_element.rs)  [数组, 哈希表, 计数]

  - LeetCode 2404. 出现最频繁的偶数元素 <https://leetcode.cn/problems/most-frequent-even-element>

- [检查相同字母间的距离](src/array/check_distances_between_same_letters.rs)  [数组, 哈希表, 字符串]

  - LeetCode 2399. 检查相同字母间的距离 <https://leetcode.cn/problems/check-distances-between-same-letters>

- [合并石头的最低成本](src/array/minimum_cost_to_merge_stones.rs)  [数组, 动态规划]

  - LeetCode 1000. 合并石头的最低成本 <https://leetcode.cn/problems/minimum-cost-to-merge-stones>

- [交换一次的先前排列](src/array/previous_permutation_with_one_swap.rs)  [贪心, 数组]

  - LeetCode 1053. 交换一次的先前排列 <https://leetcode.cn/problems/previous-permutation-with-one-swap>

- [多边形三角剖分的最低得分](src/array/minimum_score_triangulation_of_polygon.rs)  [数组, 动态规划]

  - LeetCode 1039. 多边形三角剖分的最低得分 <https://leetcode.cn/problems/minimum-score-triangulation-of-polygon>

- [算术三元组的数目](src/array/number_of_arithmetic_triplets.rs)  [数组, 哈希表, 双指针, 枚举]

  - LeetCode 2367. 算术三元组的数目 <https://leetcode.cn/problems/number-of-arithmetic-triplets>

- [和相等的子数组](src/array/find_subarrays_with_equal_sum.rs)  [数组, 哈希表]

  - LeetCode 2395. 和相等的子数组 <https://leetcode.cn/problems/find-subarrays-with-equal-sum>

- [和等于 k 的最长子数组长度](src/array/maximum_size_subarray_sum_equals_k.rs)  [数组, 哈希表]

  - LeetCode 325. 和等于 k 的最长子数组长度 <https://leetcode.cn/problems/maximum-size-subarray-sum-equals-k/>

- [提莫攻击](src/array/teemo_attacking.rs)  [数组, 模拟]

  - LeetCode 495. 提莫攻击 <https://leetcode.cn/problems/teemo-attacking/>

- [单调数列](src/array/monotonic_array.rs)  [数组]

  - LeetCode 896. 单调数列 <https://leetcode.cn/problems/monotonic-array/>

- [统计好三元组](src/array/count_good_triplets.rs)  [数组, 枚举]

  - LeetCode 1534. 统计好三元组 <https://leetcode.cn/problems/count-good-triplets/>

- [翻转对](src/array/reverse_pairs.rs)  [树状数组, 线段树, 数组, 二分查找, 分治, 有序集合, 归并排序]

  - LeetCode 493. 翻转对 <https://leetcode.cn/problems/reverse-pairs/>

- [合并区间](src/array/merge_intervals.rs)  [数组, 排序]

  - LeetCode 56. 合并区间 <https://leetcode.cn/problems/merge-intervals/>

- [零矩阵](src/array/zero_matrix_lcci.rs)  [数组, 哈希表, 矩阵]

  - LeetCode 面试题 01.08. 零矩阵 <https://leetcode.cn/problems/zero-matrix-lcci>

- [数组中的第K个最大元素](src/array/kth_largest_element_in_an_array.rs)  [数组, 分治, 快速选择, 排序, 堆（优先队列）]

  - LeetCode 215. 数组中的第K个最大元素 <https://leetcode.cn/problems/kth-largest-element-in-an-array/>

- [数组的相对排序](src/array/relative_sort_array.rs)  [数组, 哈希表, 计数排序, 排序]

  - LeetCode 1122. 数组的相对排序 <https://leetcode.cn/problems/relative-sort-array/>

- [二进制矩阵中的特殊位置](src/array/special_positions_in_a_binary_matrix.rs)  [数组, 矩阵]

  - LeetCode 1582. 二进制矩阵中的特殊位置 <https://leetcode.cn/problems/special-positions-in-a-binary-matrix>

- [无重叠区间](src/array/non_overlapping_intervals.rs)  [贪心, 数组, 动态规划, 排序]

  - LeetCode 435. 无重叠区间 <https://leetcode.cn/problems/non-overlapping-intervals/>

- [最长数对链](src/array/maximum_length_of_pair_chain.rs)  [贪心, 数组, 动态规划, 排序]

  - LeetCode 646. 最长数对链 <https://leetcode.cn/problems/maximum-length-of-pair-chain>

- [字母异位词分组](src/array/group_anagrams.rs)  [数组, 哈希表, 字符串, 排序]

  - LeetCode 49. 字母异位词分组 <https://leetcode.cn/problems/group-anagrams>

- [滑动窗口最大值](src/array/sliding_window_maximum.rs)  [队列, 数组, 滑动窗口, 单调队列, 堆（优先队列）]

  - LeetCode 239. 滑动窗口最大值 <https://leetcode.cn/problems/sliding-window-maximum>

- [两数之和](src/array/two_sum.rs)  [数组, 哈希表]

  - LeetCode 1. 两数之和 <https://leetcode.cn/problems/two-sum>

- [重新排列数组](src/array/shuffle_the_array.rs)  [数组]

  - LeetCode 1470. 重新排列数组 <https://leetcode.cn/problems/shuffle-the-array/>

- [删除有序数组中的重复项](src/array/remove_duplicates_from_sorted_array.rs)  [数组, 双指针]

  - LeetCode 26. 删除有序数组中的重复项 <https://leetcode.cn/problems/remove-duplicates-from-sorted-array/>

- [加一](src/array/plus_one.rs)  [数组, 数学]

  - LeetCode 66. 加一 <https://leetcode.cn/problems/plus-one/>

- [移动零](src/array/move_zeroes.rs)  [数组, 双指针]

  - LeetCode 283. 移动零 <https://leetcode.cn/problems/move-zeroes/>

- [通过翻转子数组使两个数组相等](src/array/make_two_arrays_equal_by_reversing_sub_arrays.rs)  [数组, 哈希表, 排序]

  - LeetCode 1460. 通过翻转子数组使两个数组相等 <https://leetcode.cn/problems/make-two-arrays-equal-by-reversing-sub-arrays>

### 栈

- [队列中可以看到的人数](src/stack/number_of_visible_people_in_a_queue.rs)  [栈, 数组, 单调栈]

  - LeetCode 1944. 队列中可以看到的人数 <https://leetcode.cn/problems/number-of-visible-people-in-a-queue>

- [从链表中移除节点](src/stack/remove_nodes_from_linked_list.rs)  [栈, 递归, 链表, 单调栈]

  - LeetCode 2487. 从链表中移除节点 <https://leetcode.cn/problems/remove-nodes-from-linked-list>

- [叶值的最小代价生成树](src/stack/minimum_cost_tree_from_leaf_values.rs)  [栈, 贪心, 数组, 动态规划, 单调栈]

  - LeetCode 1130. 叶值的最小代价生成树 <https://leetcode.cn/problems/minimum-cost-tree-from-leaf-values>

- [检查替换后的词是否有效](src/stack/check_if_word_is_valid_after_substitutions.rs)  [栈, 字符串]

  - LeetCode 1003. 检查替换后的词是否有效 <https://leetcode.cn/problems/check-if-word-is-valid-after-substitutions>

- [餐盘栈](src/stack/dinner_plate_stacks.rs)  [栈, 设计, 哈希表, 堆（优先队列）]

  - LeetCode 1172. 餐盘栈 <https://leetcode.cn/problems/dinner-plate-stacks>

- [链表中的下一个更大节点](src/stack/next_greater_node_in_linked_list.rs)  [栈, 数组, 链表, 单调栈]

  - LeetCode 1019. 链表中的下一个更大节点 <https://leetcode.cn/problems/next-greater-node-in-linked-list>

- [删除最短的子数组使剩余数组有序](src/stack/shortest_subarray_to_be_removed_to_make_array_sorted.rs)  [栈, 数组, 双指针, 二分查找, 单调栈]

  - LeetCode 1574. 删除最短的子数组使剩余数组有序 <https://leetcode.cn/problems/shortest-subarray-to-be-removed-to-make-array-sorted>

- [使括号有效的最少添加](src/stack/minimum_add_to_make_parentheses_valid.rs)  [栈, 贪心, 字符串]

  - LeetCode 921. 使括号有效的最少添加 <https://leetcode.cn/problems/minimum-add-to-make-parentheses-valid>

- [文件夹操作日志搜集器](src/stack/crawler_log_folder.rs)  [栈, 数组, 字符串]

  - LeetCode 1598. 文件夹操作日志搜集器 <https://leetcode.cn/problems/crawler-log-folder>

- [最小栈](src/stack/min_stack.rs)  [栈, 设计]

  - LeetCode 155. 最小栈 <https://leetcode.cn/problems/min-stack/>

- [有效的括号](src/stack/valid_parentheses.rs)  [栈, 字符串]

  - LeetCode 20. 有效的括号 <https://leetcode.cn/problems/valid-parentheses/>

- [验证栈序列](src/stack/validate_stack_sequences.rs)  [栈, 数组, 模拟]

  - LeetCode 946. 验证栈序列 <https://leetcode.cn/problems/validate-stack-sequences>

- [商品折扣后的最终价格](src/stack/final_prices_with_a_special_discount_in_a_shop.rs)  [栈, 数组, 单调栈]

  - LeetCode 1475. 商品折扣后的最终价格 <https://leetcode.cn/problems/final-prices-with-a-special-discount-in-a-shop>

### 树

- [字符串中的额外字符](src/tree/extra_characters_in_a_string.rs)  [字典树, 数组, 哈希表, 字符串, 动态规划]

  - LeetCode 2707. 字符串中的额外字符 <https://leetcode.cn/problems/extra-characters-in-a-string>

- [从二叉搜索树到更大和树](src/tree/binary_search_tree_to_greater_sum_tree.rs)  [树, 深度优先搜索, 二叉搜索树, 二叉树]

  - LeetCode 1038. 从二叉搜索树到更大和树 <https://leetcode.cn/problems/binary-search-tree-to-greater-sum-tree>

- [树节点的第 K 个祖先](src/tree/kth_ancestor_of_a_tree_node.rs)  [树, 深度优先搜索, 广度优先搜索, 设计, 二分查找, 动态规划]

  - LeetCode 1483. 树节点的第 K 个祖先 <https://leetcode.cn/problems/kth-ancestor-of-a-tree-node>

- [删点成林](src/tree/delete_nodes_and_return_forest.rs)  [树, 深度优先搜索, 数组, 哈希表, 二叉树]

  - LeetCode 1110. 删点成林 <https://leetcode.cn/problems/delete-nodes-and-return-forest>

- [根到叶路径上的不足节点](src/tree/insufficient_nodes_in_root_to_leaf_paths.rs)  [树, 深度优先搜索, 二叉树]

  - LeetCode 1080. 根到叶路径上的不足节点 <https://leetcode.cn/problems/insufficient-nodes-in-root-to-leaf-paths>

- [二叉搜索子树的最大键值和](src/tree/maximum_sum_bst_in_binary_tree.rs)  [树, 深度优先搜索, 二叉搜索树, 动态规划, 二叉树]

  - LeetCode 1373. 二叉搜索子树的最大键值和 <https://leetcode.cn/problems/maximum-sum-bst-in-binary-tree>

- [通知所有员工所需的时间](src/tree/time_needed_to_inform_all_employees.rs)  [树, 深度优先搜索, 广度优先搜索]

  - LeetCode 1376. 通知所有员工所需的时间 <https://leetcode.cn/problems/time-needed-to-inform-all-employees>

- [节点与其祖先之间的最大差值](src/tree/maximum_difference_between_node_and_ancestor.rs)  [树, 深度优先搜索, 二叉树]

  - LeetCode 1026. 节点与其祖先之间的最大差值 <https://leetcode.cn/problems/maximum-difference-between-node-and-ancestor>

- [驼峰式匹配](src/tree/camelcase_matching.rs)  [字典树, 双指针, 字符串, 字符串匹配]

  - LeetCode 1023. 驼峰式匹配 <https://leetcode.cn/problems/camelcase-matching>

- [二叉搜索树中的插入操作](src/tree/insert_into_a_binary_search_tree.rs)  [树, 二叉搜索树, 二叉树]

  - LeetCode 701. 二叉搜索树中的插入操作 <https://leetcode.cn/problems/insert-into-a-binary-search-tree>

- [二叉树的前序遍历](src/tree/binary_tree_preorder_traversal.rs)  [栈, 树, 深度优先搜索, 二叉树]

  - LeetCode 144. 二叉树的前序遍历 <https://leetcode.cn/problems/binary-tree-preorder-traversal/>

- [二叉树的中序遍历](src/tree/binary_tree_inorder_traversal.rs)  [栈, 树, 深度优先搜索, 二叉树]

  - LeetCode 94. 二叉树的中序遍历 <https://leetcode.cn/problems/binary-tree-inorder-traversal>

- [二叉树的后序遍历](src/tree/binary_tree_postorder_traversal.rs)  [栈, 树, 深度优先搜索, 二叉树]

  - LeetCode 145. 二叉树的后序遍历 <https://leetcode.cn/problems/binary-tree-postorder-traversal>

- [二叉树的层序遍历](src/tree/binary_tree_level_order_traversal.rs)  [树, 广度优先搜索, 二叉树]

  - LeetCode 102. 二叉树的层序遍历 <https://leetcode.cn/problems/binary-tree-level-order-traversal>

- [最长同值路径](src/tree/longest_univalue_path.rs)  [树, 深度优先搜索, 二叉树]

  - LeetCode 687. 最长同值路径 <https://leetcode.cn/problems/longest-univalue-path>

### 链表

- [在链表中插入最大公约数](src/list/insert_greatest_common_divisors_in_linked_list.rs)  [链表, 数学, 数论]

  - LeetCode 2807. 在链表中插入最大公约数 <https://leetcode.cn/problems/insert-greatest-common-divisors-in-linked-list>

- [从链表中删去总和值为零的连续节点](src/list/remove_zero_sum_consecutive_nodes_from_linked_list.rs)  [哈希表, 链表]

  - LeetCode 1171. 从链表中删去总和值为零的连续节点 <https://leetcode.cn/problems/remove-zero-sum-consecutive-nodes-from-linked-list>

- [删除链表的倒数第 N 个结点](src/list/remove_nth_node_from_end_of_list.rs)  [链表, 双指针]

  - LeetCode 19. 删除链表的倒数第 N 个结点 <https://leetcode.cn/problems/remove-nth-node-from-end-of-list/>

- [链表的中间结点](src/list/middle_of_the_linked_list.rs)  [链表, 双指针]

  - LeetCode 876. 链表的中间结点 <https://leetcode.cn/problems/middle-of-the-linked-list/>

### 图

- [重新规划路线](src/graphs/reorder_routes_to_make_all_paths_lead_to_the_city_zero.rs)  [深度优先搜索, 广度优先搜索, 图]

  - LeetCode 1466. 重新规划路线 <https://leetcode.cn/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero>

- [最小化旅行的价格总和](src/graphs/minimize_the_total_price_of_the_trips.rs)  [树, 深度优先搜索, 图, 数组, 动态规划]

  - LeetCode 2646. 最小化旅行的价格总和 <https://leetcode.cn/problems/minimize-the-total-price-of-the-trips>

- [到达首都的最少油耗](src/graphs/minimum_fuel_cost_to_report_to_the_capital.rs)  [树, 深度优先搜索, 广度优先搜索, 图]

  - LeetCode 2477. 到达首都的最少油耗 <https://leetcode.cn/problems/minimum-fuel-cost-to-report-to-the-capital>

- [T 秒后青蛙的位置](src/graphs/frog_position_after_t_seconds.rs)  [树, 深度优先搜索, 广度优先搜索, 图]

  - LeetCode 1377. T 秒后青蛙的位置 <https://leetcode.cn/problems/frog-position-after-t-seconds>

- [不邻接植花](src/graphs/flower_planting_with_no_adjacent.rs)  [深度优先搜索, 广度优先搜索, 图]

  - LeetCode 1042. 不邻接植花 <https://leetcode.cn/problems/flower-planting-with-no-adjacent>

- [找到小镇的法官](src/graphs/find_the_town_judge.rs)  [图, 数组, 哈希表]

  - LeetCode 997. 找到小镇的法官 <https://leetcode.cn/problems/find-the-town-judge>

### 排序

- [购买两块巧克力](src/sort/buy_two_chocolates.rs)  [数组, 排序]

  - LeetCode 2706. 购买两块巧克力 <https://leetcode.cn/problems/buy-two-chocolates>

- [下一个更大元素 IV](src/sort/next_greater_element_iv.rs)  [栈, 数组, 二分查找, 排序, 单调栈, 堆（优先队列）]

  - LeetCode 2454. 下一个更大元素 IV <https://leetcode.cn/problems/next-greater-element-iv>

- [出租车的最大盈利](src/sort/maximum_earnings_from_taxi.rs)  [数组, 二分查找, 动态规划, 排序]

  - LeetCode 2008. 出租车的最大盈利 <https://leetcode.cn/problems/maximum-earnings-from-taxi>

- [可被三整除的最大和](src/sort/greatest_sum_divisible_by_three.rs)  [贪心, 数组, 动态规划, 排序]

  - LeetCode 1262. 可被三整除的最大和 <https://leetcode.cn/problems/greatest-sum-divisible-by-three>

- [比较字符串最小字母出现频次](src/sort/compare_strings_by_frequency_of_the_smallest_character.rs)  [数组, 哈希表, 字符串, 二分查找, 排序]

  - LeetCode 1170. 比较字符串最小字母出现频次 <https://leetcode.cn/problems/compare-strings-by-frequency-of-the-smallest-character>

- [老鼠和奶酪](src/sort/mice_and_cheese.rs)  [贪心, 数组, 排序, 堆（优先队列）]

  - LeetCode 2611. 老鼠和奶酪 <https://leetcode.cn/problems/mice-and-cheese>

- [不同的平均值数目](src/sort/number_of_distinct_averages.rs)  [数组, 哈希表, 双指针, 排序]

  - LeetCode 2465. 不同的平均值数目 <https://leetcode.cn/problems/number-of-distinct-averages>

- [礼盒的最大甜蜜度](src/sort/maximum_tastiness_of_candy_basket.rs)  [数组, 二分查找, 排序]

  - LeetCode 2517. 礼盒的最大甜蜜度 <https://leetcode.cn/problems/maximum-tastiness-of-candy-basket>

- [受标签影响的最大值](src/sort/largest_values_from_labels.rs)  [贪心, 数组, 哈希表, 计数, 排序]

  - LeetCode 1090. 受标签影响的最大值 <https://leetcode.cn/problems/largest-values-from-labels>

- [距离相等的条形码](src/sort/distant_barcodes.rs)  [贪心, 数组, 哈希表, 计数, 排序, 堆（优先队列）]

  - LeetCode 1054. 距离相等的条形码 <https://leetcode.cn/problems/distant-barcodes>

- [与对应负数同时存在的最大正整数](src/sort/largest_positive_integer_that_exists_with_its_negative.rs)  [数组, 哈希表, 双指针, 排序]

  - LeetCode 2441. 与对应负数同时存在的最大正整数 <https://leetcode.cn/problems/largest-positive-integer-that-exists-with-its-negative>

- [按身高排序](src/sort/sort_the_people.rs)  [数组, 哈希表, 字符串, 排序]

  - LeetCode 2418. 按身高排序 <https://leetcode.cn/problems/sort-the-people>

- [使数组严格递增](src/sort/make_array_strictly_increasing.rs)  [数组, 二分查找, 动态规划, 排序]

  - LeetCode 1187. 使数组严格递增 <https://leetcode.cn/problems/make-array-strictly-increasing>

- [移动石子直到连续 II](src/sort/moving_stones_until_consecutive_ii.rs)  [数组, 数学, 双指针, 排序]

  - LeetCode 1040. 移动石子直到连续 II <https://leetcode.cn/problems/moving-stones-until-consecutive-ii>

- [两点之间不包含任何点的最宽垂直区域](src/sort/widest_vertical_area_between_two_points_containing_no_points.rs)  [数组, 排序]

  - LeetCode 1637. 两点之间不包含任何点的最宽垂直区域 <https://leetcode.cn/problems/widest-vertical-area-between-two-points-containing-no-points>

- [排序数组](src/sort/sort_an_array.rs)  [数组, 分治, 桶排序, 计数排序, 基数排序, 排序, 堆（优先队列）, 归并排序]

  - LeetCode 912. 排序数组 <https://leetcode.cn/problems/sort-an-array/>

- [合并两个有序链表](src/sort/merge_two_sorted_lists.rs)  [递归, 链表]

  - LeetCode 21. 合并两个有序链表 <https://leetcode.cn/problems/merge-two-sorted-lists/>

- [反转链表](src/sort/reverse_linked_list.rs)  [递归, 链表]

  - LeetCode 206. 反转链表 <https://leetcode.cn/problems/reverse-linked-list/>

### 其它

- [赎金信](src/map/ransom_note.rs)  [哈希表, 字符串, 计数]

  - LeetCode 383. 赎金信 <https://leetcode.cn/problems/ransom-note>

- [最小体力消耗路径](src/search/path_with_minimum_effort.rs)  [深度优先搜索, 广度优先搜索, 并查集, 数组, 二分查找, 矩阵, 堆（优先队列）]

  - LeetCode 1631. 最小体力消耗路径 <https://leetcode.cn/problems/path-with-minimum-effort>

- [爬楼梯](src/search/climbing_stairs.rs)  [记忆化搜索, 数学, 动态规划]

  - LeetCode 70. 爬楼梯 <https://leetcode.cn/problems/climbing-stairs>

- [水域大小](src/search/pond_sizes_lcci.rs)  [深度优先搜索, 广度优先搜索, 并查集, 数组, 矩阵]

  - LeetCode 面试题 16.19. 水域大小 <https://leetcode.cn/problems/pond-sizes-lcci>

- [黑白翻转棋](src/search/flip_chess.rs)  [广度优先搜索, 数组, 矩阵]

  - LeetCode LCP 41. 黑白翻转棋 <https://leetcode.cn/problems/fHi6rV>

- [统计封闭岛屿的数目](src/search/number_of_closed_islands.rs)  [深度优先搜索, 广度优先搜索, 并查集, 数组, 矩阵]

  - LeetCode 1254. 统计封闭岛屿的数目 <https://leetcode.cn/problems/number-of-closed-islands>

- [铺瓷砖](src/other/tiling_a_rectangle_with_the_fewest_squares.rs)  [动态规划, 回溯]

  - LeetCode 1240. 铺瓷砖 <https://leetcode.cn/problems/tiling-a-rectangle-with-the-fewest-squares>

- [单字符重复子串的最大长度](src/map/swap_for_longest_repeated_character_substring.rs)  [哈希表, 字符串, 滑动窗口]

  - LeetCode 1156. 单字符重复子串的最大长度 <https://leetcode.cn/problems/swap-for-longest-repeated-character-substring>

- [二进制矩阵中的最短路径](src/search/shortest_path_in_binary_matrix.rs)  [广度优先搜索, 数组, 矩阵]

  - LeetCode 1091. 二进制矩阵中的最短路径 <https://leetcode.cn/problems/shortest-path-in-binary-matrix>

- [活字印刷](src/map/letter_tile_possibilities.rs)  [哈希表, 字符串, 回溯, 计数]

  - LeetCode 1079. 活字印刷 <https://leetcode.cn/problems/letter-tile-possibilities>

- [可被 K 整除的最小整数](src/map/smallest_integer_divisible_by_k.rs)  [哈希表, 数学]

  - LeetCode 1015. 可被 K 整除的最小整数 <https://leetcode.cn/problems/smallest-integer-divisible-by-k>

- [推箱子](src/search/minimum_moves_to_move_a_box_to_their_target_location.rs)  [广度优先搜索, 数组, 矩阵, 堆（优先队列）]

  - LeetCode 1263. 推箱子 <https://leetcode.cn/problems/minimum-moves-to-move-a-box-to-their-target-location>

- [强整数](src/map/powerful_integers.rs)  [哈希表, 数学]

  - LeetCode 970. 强整数 <https://leetcode.cn/problems/powerful-integers>

- [删除字符使频率相同](src/map/remove_letter_to_equalize_frequency.rs)  [哈希表, 字符串, 计数]

  - LeetCode 2423. 删除字符使频率相同 <https://leetcode.cn/problems/remove-letter-to-equalize-frequency>

- [统计只差一个字符的子串数目](src/map/count_substrings_that_differ_by_one_character.rs)  [哈希表, 字符串, 动态规划]

  - LeetCode 1638. 统计只差一个字符的子串数目 <https://leetcode.cn/problems/count-substrings-that-differ-by-one-character>

- [第 k 个数](src/map/get_kth_magic_number_lcci.rs)  [哈希表, 数学, 动态规划, 堆（优先队列）]

  - LeetCode 面试题 17.09. 第 k 个数 <https://leetcode.cn/problems/get-kth-magic-number-lcci>

- [判定是否互为字符重排](src/map/check_permutation_lcci.rs)  [哈希表, 字符串, 排序]

  - LeetCode 面试题 01.02. 判定是否互为字符重排 <https://leetcode.cn/problems/check-permutation-lcci>

- [有效的字母异位词](src/map/valid_anagram.rs)  [哈希表, 字符串, 排序]

  - LeetCode 242. 有效的字母异位词 <https://leetcode.cn/problems/valid-anagram>

- [阶乘函数后 K 个零](src/math/preimage_size_of_factorial_zeroes_function.rs)  [数学, 二分查找]

  - LeetCode 793. 阶乘函数后 K 个零 <https://leetcode.cn/problems/preimage-size-of-factorial-zeroes-function>

- [灯泡开关 Ⅱ](src/math/bulb_switcher_ii.rs)  [位运算, 深度优先搜索, 广度优先搜索, 数学]

  - LeetCode 672. 灯泡开关 Ⅱ <https://leetcode.cn/problems/bulb-switcher-ii>

- [消失的两个数字](src/math/missing_two_lcci.rs)  [位运算, 数组, 哈希表]

  - LeetCode 面试题 17.19. 消失的两个数字 <https://leetcode.cn/problems/missing-two-lcci>

- [统计字典序元音字符串的数目](src/math/count_sorted_vowel_strings.rs)  [数学, 动态规划, 组合数学]

  - LeetCode 1641. 统计字典序元音字符串的数目 <https://leetcode.cn/problems/count-sorted-vowel-strings>

- [公因子的数目](src/math/number_of_common_factors.rs)  [数学, 枚举, 数论]

  - LeetCode 2427. 公因子的数目 <https://leetcode.cn/problems/number-of-common-factors>

- [负二进制转换](src/math/convert_to_base_2.rs)  [数学]

  - LeetCode 1017. 负二进制转换 <https://leetcode.cn/problems/convert-to-base-2>

- [最小的必要团队](src/math/smallest_sufficient_team.rs)  [位运算, 数组, 动态规划, 状态压缩]

  - LeetCode 1125. 最小的必要团队 <https://leetcode.cn/problems/smallest-sufficient-team>

- [困于环中的机器人](src/math/robot_bounded_in_circle.rs)  [数学, 字符串, 模拟]

  - LeetCode 1041. 困于环中的机器人 <https://leetcode.cn/problems/robot-bounded-in-circle>

- [统计共同度过的日子数](src/math/count_days_spent_together.rs)  [数学, 字符串]

  - LeetCode 2409. 统计共同度过的日子数 <https://leetcode.cn/problems/count-days-spent-together>

- [最小偶倍数](src/math/smallest_even_multiple.rs)  [数学, 数论]

  - LeetCode 2413. 最小偶倍数 <https://leetcode.cn/problems/smallest-even-multiple>

- [构建回文串检测](src/math/can_make_palindrome_from_substring.rs)  [位运算, 数组, 哈希表, 字符串, 前缀和]

  - LeetCode 1177. 构建回文串检测 <https://leetcode.cn/problems/can-make-palindrome-from-substring>

- [分割圆的最少切割次数](src/math/minimum_cuts_to_divide_a_circle.rs)  [几何, 数学]

  - LeetCode 2481. 分割圆的最少切割次数 <https://leetcode.cn/problems/minimum-cuts-to-divide-a-circle>

- [连通两组点的最小成本](src/math/minimum_cost_to_connect_two_groups_of_points.rs)  [位运算, 数组, 动态规划, 状态压缩, 矩阵]

  - LeetCode 1595. 连通两组点的最小成本 <https://leetcode.cn/problems/minimum-cost-to-connect-two-groups-of-points>

- [最大化网格幸福感](src/math/maximize_grid_happiness.rs)  [位运算, 记忆化搜索, 动态规划, 状态压缩]

  - LeetCode 1659. 最大化网格幸福感 <https://leetcode.cn/problems/maximize-grid-happiness>

- [圆和矩形是否有重叠](src/math/circle_and_rectangle_overlapping.rs)  [几何, 数学]

  - LeetCode 1401. 圆和矩形是否有重叠 <https://leetcode.cn/problems/circle-and-rectangle-overlapping>

- [找出中枢整数](src/math/find_the_pivot_integer.rs)  [数学, 前缀和]

  - LeetCode 2485. 找出中枢整数 <https://leetcode.cn/problems/find-the-pivot-integer>

- [下一个更大的数值平衡数](src/math/next_greater_numerically_balanced_number.rs)  [数学, 回溯, 枚举]

  - LeetCode 2048. 下一个更大的数值平衡数 <https://leetcode.cn/problems/next-greater-numerically-balanced-number>

- [不浪费原料的汉堡制作方案](src/math/number_of_burgers_with_no_waste_of_ingredients.rs)  [数学]

  - LeetCode 1276. 不浪费原料的汉堡制作方案 <https://leetcode.cn/problems/number-of-burgers-with-no-waste-of-ingredients>

- [参加考试的最大学生数](src/math/maximum_students_taking_exam.rs)  [位运算, 数组, 动态规划, 状态压缩, 矩阵]

  - LeetCode 1349. 参加考试的最大学生数 <https://leetcode.cn/problems/maximum-students-taking-exam>

- [一周中的第几天](src/math/day_of_the_week.rs)  [数学]

  - LeetCode 1185. 一周中的第几天 <https://leetcode.cn/problems/day-of-the-week>

- [被列覆盖的最多行数](src/math/maximum_rows_covered_by_columns.rs)  [位运算, 数组, 回溯, 枚举, 矩阵]

  - LeetCode 2397. 被列覆盖的最多行数 <https://leetcode.cn/problems/maximum-rows-covered-by-columns>
