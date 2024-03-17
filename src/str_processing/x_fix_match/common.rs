//! 通用逻辑
//! * 🎯【2024-03-17 17:43:49】用于存储「数组（切片）搜索」算法
//!   * 📌选择性依赖[`crate::algorithms`]包

use std::cmp::Ordering;

/// 搜索 | 使用二分查找
/// * 📌内部使用的搜索函数
#[cfg(feature = "algorithms")]
#[inline(always)]
pub(super) fn search_by<T1, T2, F>(arr: &[T1], target: &T2, cmp: F) -> Result<usize, usize>
where
    F: Fn(&T2, &T1) -> Ordering,
{
    // 重定向到「二分查找」
    crate::algorithms::binary_search_by(arr, target, cmp)
}

/// 搜索 | 使用线性查找
/// * 📌内部默认使用的搜索函数
#[cfg(not(feature = "algorithms"))]
#[inline(always)]
pub fn search_by<T1, T2, F>(arr: &[T1], target: &T2, cmp: F) -> Result<usize, usize>
where
    F: Fn(&T2, &T1) -> Ordering,
{
    // 重定向到「线性查找」
    linear_search_by(arr, target, cmp)
}

/// 线性查找（使用「判据函数」比对大小）
/// * 🎯「前后缀匹配」在没使用[`crate::algorithms`]时的默认算法
/// * 🎯用于对某个**已排好序**的元素的查找
///   * 由此可用于从零渐近构造有序序列
/// * 🎯找到某个元素的位置，或至少反映「它应该被插入的位置」
///   * 此处「应该被插入的位置」指的是「插入之后它的索引」
///   * 亦即「插入之后会把当前位置的元素后移」
///   * 或「第一个小于该元素」的位置
///   * 📌核心在「插入后保持『比自己大的 > 自己 > 已存在』的顺序」
pub fn linear_search_by<T1, T2, F>(arr: &[T1], target: &T2, cmp: F) -> Result<usize, usize>
where
    F: Fn(&T2, &T1) -> Ordering,
{
    for (i, existed) in arr.iter().enumerate() {
        match cmp(target, existed) {
            // 自己 = 已存在 ⇒ 如果等于，直接返回这个位置
            Ordering::Equal => return Ok(i),
            // 自己 < 已存在 ⇒ 确保匹配到「第一个比自己小的」然后替代它的位置
            // * 📌保证插入后「比自己大的 > 自己 > 已存在」
            Ordering::Less => return Err(i),
            // 自己 < 已存在 ⇒ 继续
            Ordering::Greater => (),
        }
    }
    // 否则插入末尾
    Err(arr.len())
}
