/// 存储一些常用的辅助算法
/// * 🎯不依赖标准库
use std::cmp::Ordering;

/// 二分查找
/// * 🎯用于对某个**已排好序**的元素的查找
///   * 由此可用于从零渐近构造有序序列
/// * 🎯找到某个元素的位置，或至少反映「它应该被插入的位置」
///   * 此处「应该被插入的位置」指的是「插入之后它的索引」
///   * 亦即「插入之后会把当前位置的元素后移」
///   * 或「第一个大于该元素」的位置
pub fn binary_search<T>(arr: &[T], target: &T) -> Result<usize, usize>
where
    T: std::cmp::Ord,
{
    // 初始化左右边界
    let mut left = 0;
    let mut right = arr.len() - 1;
    // 预先初始化
    let mut mid = left + (right - left) / 2;
    while left <= right {
        mid = left + (right - left) / 2;
        match target.cmp(&arr[mid]) {
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
    Err(if arr[mid] < *target { mid + 1 } else { mid })
}

/// 单元测试
#[cfg(test)]
mod tests {

    use std::fmt::Debug;

    use super::*;

    /// 单测/二分查找/整数分派
    fn _test_binary_search_usize(arr: &mut [usize]) {
        _test_binary_search(arr, *arr.first().unwrap()..*arr.last().unwrap());
    }
    fn _test_binary_search_isize(arr: &mut [isize]) {
        _test_binary_search(arr, *arr.first().unwrap()..*arr.last().unwrap());
    }
    fn _test_binary_search_char(arr: &mut [char]) {
        _test_binary_search(arr, *arr.first().unwrap()..*arr.last().unwrap());
    }

    /// 单测/二分查找/通用
    fn _test_binary_search<T>(arr: &mut [T], boarder_range: impl IntoIterator<Item = T>)
    where
        T: Ord + Debug,
    {
        // 先排序
        arr.sort();
        // 成功查找
        for (i, target) in arr.iter().enumerate() {
            let res = binary_search(arr, target);
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
            let res = binary_search(arr, &target);
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

    /// 单测/二分查找
    #[test]
    fn test_binary_search() {
        // 构造并测试数组 //
        // 简单数组
        _test_binary_search_usize(&mut [2, 4, 6, 7, 8]);
        _test_binary_search_usize(&mut [1, 3, 5, 7, 9]);
        _test_binary_search_usize(&mut [0, 0, 0, 0, 0]); // 重复元素
        _test_binary_search_usize(&mut std::array::from_fn::<_, 100, _>(|i| i * i));
        // _test_binary_search_usize(&mut (0..10000).map(|x| 2 * x).collect::<Vec<_>>());
        for gap in 1..=100 {
            _test_binary_search_usize(&mut (0..10000).filter(|x| x % gap == 0).collect::<Vec<_>>());
        }

        // 涉及负数 | ⚠️注意：直接对数组切片调用sort无效
        _test_binary_search_isize(&mut [-2, -4, -6, -7, -8]);
        _test_binary_search_isize(&mut [-1, -3, -5, -7, -9]);
        _test_binary_search_isize(&mut [0, -0, 0, -0, 0]); // 重复元素
        _test_binary_search_isize(
            &mut (0..10000)
                .map(|x| if x & 1 == 0 { x } else { -x })
                .collect::<Vec<_>>(),
        );

        // 其它可比类型 | 字符
        _test_binary_search_char(&mut ['a', 'b', 'f', '你', '好', '😋', '✨']); // 重复元素
        _test_binary_search_char(&mut "我们有权报复三体文明".chars().collect::<Vec<_>>()); // 重复元素
        _test_binary_search_char(&mut ('\x00'..'\u{00ff}').collect::<Vec<_>>());

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
        _test_binary_search(&mut strings, strings_more);

        // 规则数组
    }
}
