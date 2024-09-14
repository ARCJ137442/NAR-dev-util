//! 预引入的内容
//! * 🎯用于【预先引入】一些「最低必要依赖」
//!   * 📌原则：**最低可用功能**
//! * 📄case: 当[`crate::vec_tools`]未启用时，使用线性查找[`crate::linear_search_by`]
//!   * 🚩由此将「线性查找」作为「最低必要依赖」

use std::cmp::Ordering;

// 默认查找算法 //

pub fn linear_search<T>(arr: &[T], target: &T) -> Result<usize, usize>
where
    T: Ord,
{
    linear_search_by(arr, target, T::cmp)
}

/// 【默认方法】线性查找（使用「判据函数」比对大小）
/// * 🎯「前后缀匹配」在没使用[`crate::vec_tools`]时的默认算法
/// * 🎯用于对某个**已排好序**的元素的查找
///   * 由此可用于从零渐近构造有序序列
/// * 🎯找到某个元素的位置，或至少反映「它应该被插入的位置」
///   * 此处「应该被插入的位置」指的是「插入之后它的索引」
///   * 亦即「插入之后会把当前位置的元素后移」
///   * 或「第一个小于该元素」的位置
///   * 📌核心在「插入后保持『比自己大的 > 自己 > 已存在』的顺序」
pub fn linear_search_by<T, Target, F>(arr: &[T], target: &Target, cmp: F) -> Result<usize, usize>
where
    F: Fn(&Target, &T) -> Ordering,
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

/// 搜索 | 使用二分查找
/// * 📌内部使用的搜索函数
#[cfg(feature = "vec_tools")]
#[inline(always)]
#[allow(dead_code)] // * 📄最初于`cargo publish`中发现
pub(crate) fn search_by<T, Target, F>(arr: &[T], target: &Target, cmp: F) -> Result<usize, usize>
where
    F: Fn(&Target, &T) -> Ordering,
{
    // 重定向到「二分查找」
    crate::vec_tools::search::binary_search_by(arr, target, cmp)
}

/// 搜索 | 使用线性查找
/// * 📌内部默认使用的搜索函数
#[cfg(not(feature = "vec_tools"))]
#[inline(always)]
#[allow(dead_code)] // * 📄最初于`cargo publish`中发现
pub(crate) fn search_by<T, Target, F>(arr: &[T], target: &Target, cmp: F) -> Result<usize, usize>
where
    F: Fn(&Target, &T) -> Ordering,
{
    // 重定向到「线性查找」
    crate::linear_search_by(arr, target, cmp)
}

/// 单元测试
#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::fmt::Debug;

    /// 单测/查找/可slice类型分派
    #[macro_export]
    macro_rules! test_search_slice {
        ($f:expr, $arr:expr $(,)?) => {
            __test_search($f, $arr, *$arr.first().unwrap()..*$arr.last().unwrap())
        };
    }

    // 查找算法 //

    /// 通用：单测/查找/单数组测试
    pub(crate) fn __test_search<T, Search>(
        search: Search,
        arr: &mut [T],
        boarder_range: impl IntoIterator<Item = T>,
    ) where
        T: Ord + Debug,
        Search: Fn(&[T], &T) -> Result<usize, usize>,
        // Cmp: Fn(&T, &T) -> Ordering,
    {
        // 先排序
        arr.sort();
        // 成功查找
        for (i, target) in arr.iter().enumerate() {
            let res = search(arr, target);
            // ! 不能使用「找到⇒找到的索引==当前位置索引」的假设：有可能会有重复的元素
            assert!(
                // 相对地，使用「找到的元素一样」
                arr[res.unwrap()] == arr[i],
                "Error on target={target:?} and res={res:?}"
            );
        }
        // 遍历查找
        for target in boarder_range {
            // 默认结果「是否有」
            let found = arr.iter().any(|item| *item == target);
            // 算法结果
            let res = search(arr, &target);
            // 判断结果是否一致
            assert_eq!(res.is_ok(), found);
            // 当查找失败时
            if !found {
                // 验证结果：是否的确是插入「第一个大于等于该元素」的位置
                // ! ⚠️↓这实际上就类似`index_of`
                // let first_greater_i = arr.iter().position(|&item| item >= target).unwrap();
                // show!(target, found, res, first_greater_i;);
                let should_insert_to = res.unwrap_err();
                // ! ⚠️有可能在边界外
                assert!(should_insert_to >= arr.len() || arr[should_insert_to] >= target);
            }
        }
        // 输出结果信息
        print!("test succeed on ");
        match arr.len() {
            0..=1000 => println!("{arr:?}"),
            l => println!(
                "[{:?}, {:?}, ..., {:?}; {l}]",
                arr[0],
                arr[1],
                arr.last().unwrap()
            ),
        }
    }

    /// 测试/单个搜索算法的测试集
    ///
    /// ! ⚠️不能对「带泛型参数的函数」进行【可能有类型多态】的传入
    /// * 📌简而言之：无法传入「带泛型参数的函数」
    /// * 因此只能用宏实现。。
    #[macro_export(local_inner_macros)]
    macro_rules! test_search {
        ($search:expr) => {

        // pub(crate) fn test_search<T, Search>(search: Search)
        // where
        //     Search: Fn(&[T], &T) -> Result<usize, usize>,
        // {
            // 构造并测试数组 //
            // 简单数组
            test_search_slice!($search, &mut [2, 4, 6, 7, 8]);
            test_search_slice!($search, &mut [1, 3, 5, 7, 9]);
            test_search_slice!($search, &mut [0, 0, 0, 0, 0]); // 重复元素
            test_search_slice!($search, &mut std::array::from_fn::<_, 100, _>(|i| i * i));
            // test_search_slice!($search, &mut (0..10000).map(|x| 2 * x).collect::<Vec<_>>());
            for gap in 1..=100 {
                test_search_slice!(
                    $search,
                    &mut (0..10000).filter(|x| x % gap == 0).collect::<Vec<_>>()
                );
            }

            // 涉及负数 | ⚠️注意：直接对数组切片调用sort无效
            test_search_slice!($search, &mut [-2, -4, -6, -7, -8]);
            test_search_slice!($search, &mut [-1, -3, -5, -7, -9]);
            test_search_slice!($search, &mut [0, -0, 0, -0, 0]); // 重复元素
            test_search_slice!(
                $search,
                &mut (0..10000)
                    .map(|x| if x & 1 == 0 { x } else { -x })
                    .collect::<Vec<_>>(),
            );

            // 其它可比类型 | 字符
            test_search_slice!($search, &mut ['a', 'b', 'f', '你', '好', '😋', '✨']); // 重复元素
            test_search_slice!(
                $search,
                &mut "我们有权报复三体文明".chars().collect::<Vec<_>>()
            ); // 重复元素
            test_search_slice!($search, &mut ('\x00'..'\u{00ff}').collect::<Vec<_>>());

            // 其它可比类型 | 字符串
            let mut strings = "\
            Self {
                prefixes: prefixes
                    .into_iter()
                    .map(|into_s| into_s.into())
                    .collect::<Vec<String>>(),
            }"
            .split_whitespace()
            .collect::<Vec<_>>();
            let strings_more =
                "pub fn new(prefixes: impl IntoIterator<Item = impl Into<String>>) -> Self {
                // ? 或许也可以「先新建空值，然后逐个添加」来实现，复杂度 ∑ 1 log 1 ~ n log n
                Self {
                    prefixes: prefixes
                        .into_iter()
                        .map(|into_s| into_s.into())
                        .collect::<Vec<String>>(),
                }
            }"
                .split_whitespace()
                .collect::<Vec<_>>();
            __test_search($search, &mut strings, strings_more);

        };
    }

    /// 实际测试/顺序查找
    #[test]
    fn test_linear_search() {
        test_search!(linear_search);
    }
}
