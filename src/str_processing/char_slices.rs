//! 用来封装与「字符数组切片」有关的工具函数
//! * 🎯用于【基于字符数组切片】的「词法Narsese」解析

/// 在「字符数组切片」中判断「是否有字符串前缀」
pub fn char_slice_has_prefix(slice: &[char], prefix: &str) -> bool {
    // // 空字串の特殊情况
    // if_return! { prefix.is_empty() => true }
    // // 先将字符串转换为「字符数组」
    // let prefix = prefix.chars().collect::<Vec<_>>();
    // // 然后验证长度（以防panic）并直接切片判等
    // prefix.len() <= slice.len() && slice[..prefix.len()] == prefix
    // * 📝【2024-03-17 00:59:10】此处求简，将「字符数组切片」变成字符串
    String::from_iter(slice).starts_with(prefix)
}

/// 在「字符数组切片」中判断「是否有字符串后缀」
pub fn char_slice_has_suffix(slice: &[char], suffix: &str) -> bool {
    // // 空字串の特殊情况
    // if_return! { suffix.is_empty() => true }
    // // 先将字符串转换为「字符数组」
    // let suffix = suffix.chars().collect::<Vec<_>>();
    // // 然后验证长度（以防panic）
    // if_return! { suffix.len() > slice.len() => false }
    // // 切片判等
    // slice[(slice.len() - suffix.len())..] == suffix
    // * 📝【2024-03-17 00:59:10】此处求简，将「字符数组切片」变成字符串
    String::from_iter(slice).ends_with(suffix)
}

/// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    /// 字符数组切片/前后缀计算
    #[test]
    fn test_char_slice_has_fix() {
        asserts! {
            show!(char_slice_has_prefix(&['a', 'b', 'c'], ""))
            show!(char_slice_has_prefix(&['a', 'b', 'c'], "a"))
            show!(char_slice_has_prefix(&['a', 'b', 'c'], "ab"))
            show!(char_slice_has_prefix(&['a', 'b', 'c'], "abc"))

            show!(char_slice_has_suffix(&['a', 'b', 'c'], ""))
            show!(char_slice_has_suffix(&['a', 'b', 'c'], "c"))
            show!(char_slice_has_suffix(&['a', 'b', 'c'], "bc"))
            show!(char_slice_has_suffix(&['a', 'b', 'c'], "abc"))
        }
    }
}
