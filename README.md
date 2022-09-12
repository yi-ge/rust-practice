# Rust练习

[![license](https://img.shields.io/github/license/yi-ge/rust-practice.svg?style=flat-square)](https://github.com/yi-ge/rust-practice/blob/master/LICENSE)
[![GitHub Actions](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fyi-ge%2Frust-practice%2Fbadge%3Fref%3Dmain&style=flat-square)](https://actions-badge.atrox.dev/yi-ge/rust-practice/goto?ref=main)
[![Test Results](https://gist.github.com/yi-ge/00fdcacb47689d14b8e9fdf7fb0f7288/raw/badge.svg)](https://github.com/yi-ge/rust-practice)
[![Coveralls github](https://img.shields.io/coveralls/github/yi-ge/rust-practice?style=flat-square)](https://coveralls.io/github/yi-ge/rust-practice?branch=main)
[![GitHub last commit](https://img.shields.io/github/last-commit/yi-ge/rust-practice.svg?style=flat-square)](https://github.com/yi-ge/rust-practice)
[![Gitpod Ready-to-Code](https://img.shields.io/badge/Gitpod-Ready--to--Code-blue?logo=gitpod&style=flat-square)](https://gitpod.io/#https://github.com/yi-ge/rust-practice)

Rust 基础算法、数据结构练习，包含 LeetCode 或其它算法练习记录。

此为个人练习仓库，代码中对重要思想进行了注释，会尽量补充解题思路。

每一道题都对应写有测试用例，但可能不够完整。如果您发现错误，欢迎给我留言，谢谢！

安装以下测试环境后，运行`yarn start`可以自动从LeetCode获取代码函数和用例说明。保存文件后将自动同步到浏览器。

特别说明：题目截图仅为了方便在代码编辑器中直接预览从而优化编码体验，题目以LeetCode官方页面为准，题目著作权及其他权利以LeetCode官方说明为准或属于LeetCode。请大家尊重版权，共同维护良好网络环境。

## 测试环境

安装最新版[Rust](https://www.rust-lang.org/)和[Node.js](https://nodejs.org)。安装完成后执行`yarn`安装依赖。

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
cargo xtask install
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
<!-- - [插入排序](src/sort/insert_sort.rs) -->
<!-- - [选择排序](src/sort/select_sort.rs) -->
<!-- - [快速排序](src/sort/quick_sort.rs) -->
<!-- - [归并排序](src/sort/merge_sort.rs) -->

## 基础数据结构

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

## 算法题

### 字符串

- [检查单词是否为句中其他单词的前缀](src/string/check_if_a_word_occurs_as_a_prefix_of_any_word_in_a_sentence.rs)  [字符串, 字符串匹配]

  - LeetCode 1455. 检查单词是否为句中其他单词的前缀 <https://leetcode.cn/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence>

- [重新格式化字符串](src/string/reformat_the_string.rs)  [字符串]

  - LeetCode 1417. 重新格式化字符串 <https://leetcode.cn/problems/reformat-the-string>

### 数组/队列/集合/映射

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

- [删除链表的倒数第 N 个结点](src/list/remove_nth_node_from_end_of_list.rs)  [链表, 双指针]

  - LeetCode 19. 删除链表的倒数第 N 个结点 <https://leetcode.cn/problems/remove-nth-node-from-end-of-list/>

- [链表的中间结点](src/list/middle_of_the_linked_list.rs)  [链表, 双指针]

  - LeetCode 876. 链表的中间结点 <https://leetcode.cn/problems/middle-of-the-linked-list/>

### 图

- [找到小镇的法官](src/graphs/find_the_town_judge.rs)  [图, 数组, 哈希表]

  - LeetCode 997. 找到小镇的法官 <https://leetcode.cn/problems/find-the-town-judge>

### 排序

- [合并两个有序链表](src/sort/merge_two_sorted_lists.rs)  [递归, 链表]

  - LeetCode 21. 合并两个有序链表 <https://leetcode.cn/problems/merge-two-sorted-lists/>

- [反转链表](src/sort/reverse_linked_list.rs)  [递归, 链表]

  - LeetCode 206. 反转链表 <https://leetcode.cn/problems/reverse-linked-list/>

### 其它

- [有效的字母异位词](src/map/valid_anagram.rs)  [哈希表, 字符串, 排序]

  - LeetCode 242. 有效的字母异位词 <https://leetcode.cn/problems/valid-anagram>

- [阶乘函数后 K 个零](src/math/preimage_size_of_factorial_zeroes_function.rs)  [数学, 二分查找]

  - LeetCode 793. 阶乘函数后 K 个零 <https://leetcode.cn/problems/preimage-size-of-factorial-zeroes-function>
