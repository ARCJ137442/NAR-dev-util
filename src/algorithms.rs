//! 存储一些常用的辅助算法
//! * 🎯不依赖外部库
use crate::if_return;
use std::cmp::{Ord, Ordering};

// /// 查找「对Ord选择算法」
// #[inline(always)]
// pub fn search_for_ord<T, Search, Cmp>(search: Search, arr: &[T], target: &T) -> Result<usize, usize>
// where
//     T: Ord,
//     Search: Fn(&[T], &T, Cmp) -> Result<usize, usize>,
//     Cmp: Fn(&T, &T) -> Ordering,
// {
//     search(arr, target, T::cmp) // ! 不能直接传入函数指针：类型不匹配
//     // search(arr, target, |target: &T, existed: &T| target.cmp(existed)) // ! 不能直接传入闭包：类型仍然不匹配
//     ; // * 【2024-03-17 21:24:53】❌结论：放弃
// }

/// 二分查找
/// * 🎯用于对某个**已排好序**的元素的查找
///   * 由此可用于从零渐近构造有序序列
/// * 🎯找到某个元素的位置，或至少反映「它应该被插入的位置」
///   * 此处「应该被插入的位置」指的是「插入之后它的索引」
///   * 亦即「插入之后会把当前位置的元素后移」
///   * 或「第一个大于该元素」的位置
/// * 🚩现在直接使用[`T::cmp`]内联到「带判据二分查找」
#[inline(always)]
pub fn binary_search<T>(arr: &[T], target: &T) -> Result<usize, usize>
where
    T: Ord,
{
    binary_search_by(arr, target, T::cmp)
}

/// 二分查找（使用「判据函数」比对大小）
/// * 🎯用于对某个**已排好序**的元素的查找
///   * 由此可用于从零渐近构造有序序列
/// * 🎯找到某个元素的位置，或至少反映「它应该被插入的位置」
///   * 📌原则：插入之后不会改变元素顺序
///   * 此处「应该被插入的位置」指的是「插入之后它的索引」
///   * 亦即「插入之后会把当前位置的元素后移」
///   * 或「第一个大于该元素」的位置
/// * 🚩【2024-03-15 16:42:44】泛化：将「有序大小判断」封装到函数`cmp`中
///   * ✨这样不再需要约束「数组元素」「目标」的类型
pub fn binary_search_by<T1, T2, Cmp>(arr: &[T1], target: &T2, cmp: Cmp) -> Result<usize, usize>
where
    Cmp: Fn(&T2, &T1) -> Ordering,
{
    // 考虑「长度为零」的特殊情况：直接返回「应该插入第一个」
    if_return! { arr.is_empty() => Err(0) }
    // 初始化左右边界
    let mut left = 0;
    let mut right = arr.len() - 1;
    // 预先初始化
    let mut mid = left + (right - left) / 2;
    while left <= right {
        mid = left + (right - left) / 2;
        // ! 此处必须是「『目标』与『已有』」比大小
        match cmp(target, &arr[mid]) {
            // 相等⇒直接返回
            Ordering::Equal => return Ok(mid),
            // 大于⇒左边界缩小
            Ordering::Greater => left = mid + 1,
            // 小于⇒目标在左边⇒右边界缩小（需要判断是否为零，避免数字溢出）
            Ordering::Less => match mid == 0 {
                true => break,
                false => right = mid - 1, // ? 到底要不要`-1`？前边的`/2`倾向于向前取值，可能导致边界取不到
            },
        }
    }
    // 找不到⇒返回「应该插入的位置」 | ⚠️【2024-03-15 10:51:34】此处可能会有一个索引的偏差
    Err(match cmp(target, &arr[mid]) == Ordering::Greater {
        true => mid + 1,
        false => mid,
    })
}

/// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{prelude::tests::__test_search, test_search};

    /// 单测/二分查找
    #[test]
    fn test_binary_search() {
        test_search!(binary_search);
    }
}
