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

## 测试环境

安装最新版[Rust](https://www.rust-lang.org/)和[Node.js](https://nodejs.org)。安装完成后执行`yarn`安装依赖。

## 基础排序算法

- [插入排序](src/sort/insert_sort.cpp)
- [冒泡排序](src/sort/bubble_sort.cpp)
- [选择排序](src/sort/select_sort.cpp)
- [快速排序](src/sort/quick_sort.cpp)
- [归并排序](src/sort/merge_sort.cpp)

## 基础数据结构

## 算法题

### 字符串

- [检查单词是否为句中其他单词的前缀](src/string/check_if_a_word_occurs_as_a_prefix_of_any_word_in_a_sentence.rs)  [字符串, 字符串匹配]

  - LeetCode 1455. 检查单词是否为句中其他单词的前缀 <https://leetcode.cn/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence>

- [重新格式化字符串](src/string/reformat_the_string.rs)  [字符串]

  - LeetCode 1417. 重新格式化字符串 <https://leetcode.cn/problems/reformat-the-string>

### 数组/队列/集合/映射

- [删除有序数组中的重复项](src/array/remove_duplicates_from_sorted_array.rs)  [数组, 双指针]

  - LeetCode 26. 删除有序数组中的重复项 <https://leetcode.cn/problems/remove-duplicates-from-sorted-array/>

- [加一](src/array/plus_one.rs)  [数组, 数学]

  - LeetCode 66. 加一 <https://leetcode.cn/problems/plus-one/>

- [移动零](src/array/move_zeroes.rs)  [数组, 双指针]

  - LeetCode 283. 移动零 <https://leetcode.cn/problems/move-zeroes/>

- [通过翻转子数组使两个数组相等](src/array/make_two_arrays_equal_by_reversing_sub_arrays.rs)  [数组, 哈希表, 排序]

  - LeetCode 1460. 通过翻转子数组使两个数组相等 <https://leetcode.cn/problems/make-two-arrays-equal-by-reversing-sub-arrays>

### 栈

### 树

### 链表

### 图

### 排序

### 其它

- [最小栈](src/heap/min_stack.rs)  [栈, 设计]

  - LeetCode 155. 最小栈 <https://leetcode.cn/problems/min-stack/>

- [有效的括号](src/heap/valid_parentheses.rs)  [栈, 字符串]

  - LeetCode 20. 有效的括号 <https://leetcode.cn/problems/valid-parentheses/>
