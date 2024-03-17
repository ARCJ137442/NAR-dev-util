//! 用来对二元组
//! * 🎯用于在「词法解析器」中直接对「字符串元组」做前缀匹配

use super::traits::*;

/// 简记 @ 二元组
type T2<T> = (T, T);

// 实现 @ (String, String) //

/// 简记 @ 字符串二元组
type T2String = T2<String>;

impl PrefixMatch<T2String> for T2String {
    fn get_prefix_from_term(term: &T2String) -> &PrefixStr {
        term.0.as_str()
    }

    // * 【2024-03-17 16:41:01】不再为特征实现不必要的「插入」逻辑
    // * ✅因此「直接替换掉自身」的怪异实现不再出现

    fn prefix_terms<'a>(&'a self) -> impl Iterator<Item = &'a T2String> + 'a
    where
        T2String: 'a,
    {
        [self].into_iter()
    }
}
impl SuffixMatch<T2String> for T2String {
    fn get_suffix_from_term(term: &T2String) -> &SuffixStr {
        term.1.as_str()
    }

    // * 【2024-03-17 16:41:01】不再为特征实现不必要的「插入」逻辑
    // * ✅因此「直接替换掉自身」的怪异实现不再出现

    fn suffix_terms<'a>(&'a self) -> impl Iterator<Item = &'a T2String> + 'a
    where
        T2String: 'a,
    {
        [self].into_iter()
    }
}

// 实现 @ (&str, &str) //

/// 简记 @ 静态字串二元组
type T2RefStr<'a> = T2<&'a str>;

impl<'s> PrefixMatch<T2RefStr<'s>> for T2RefStr<'s> {
    fn get_prefix_from_term(term: &T2RefStr<'s>) -> &'s PrefixStr {
        term.0
    }

    // * 【2024-03-17 16:41:01】不再为特征实现不必要的「插入」逻辑
    // * ✅因此「直接替换掉自身」的怪异实现不再出现

    fn prefix_terms<'a>(&'a self) -> impl Iterator<Item = &'a T2RefStr<'s>> + 'a
    where
        T2RefStr<'s>: 'a,
    {
        [self].into_iter()
    }
}
impl<'s> SuffixMatch<T2RefStr<'s>> for T2RefStr<'s> {
    fn get_suffix_from_term(term: &T2RefStr<'s>) -> &'s SuffixStr {
        term.1
    }

    // * 【2024-03-17 16:41:01】不再为特征实现不必要的「插入」逻辑
    // * ✅因此「直接替换掉自身」的怪异实现不再出现

    fn suffix_terms<'a>(&'a self) -> impl Iterator<Item = &'a T2RefStr<'s>> + 'a
    where
        T2RefStr<'s>: 'a,
    {
        [self].into_iter()
    }
}

/// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test_match_prefix, test_match_suffix};

    /// 测试 @ String二元组
    #[test]
    fn test_string() {
        // 构造
        let mut tuple: T2String = ("a".into(), "c".into());
        // 前缀匹配
        test_match_prefix! {
            tuple;
            "abc" => Some("c")
            "alpha" => Some("c")
            "argon" => Some("c")
        }
        // 后缀匹配
        test_match_suffix! {
            tuple;
            "func" => Some("a")
            "sync" => Some("a")
            "panic" => Some("a")
        }
        // 修改
        tuple = ("A".into(), "C".into());
        test_match_prefix! {
            tuple;
            "Alpha" => Some("C")
            "A, B, C" => Some("C")
            "Aaron" => Some("C")
            "Arc" => Some("C")
            "ARCJ137442" => Some("C")
        }
        test_match_suffix! {
            tuple;
            "INC" => Some("A")
            "SYNC" => Some("A")
            "A, B, C" => Some("A")
            "BASIC" => Some("A")
            "Objective-C" => Some("A")
        }
    }

    /// 测试 @ &str二元组
    #[test]
    fn test_ref_str() {
        // 构造
        let mut tuple: T2RefStr = ("a", "c");
        // 前缀匹配
        test_match_prefix! {
            tuple;
            "abc" => Some("c")
            "alpha" => Some("c")
            "argon" => Some("c")
        }
        // 后缀匹配
        test_match_suffix! {
            tuple;
            "func" => Some("a")
            "sync" => Some("a")
            "panic" => Some("a")
        }
        // 修改
        tuple = ("A", "C");
        test_match_prefix! {
            tuple;
            "Alpha" => Some("C")
            "A, B, C" => Some("C")
            "Aaron" => Some("C")
            "Arc" => Some("C")
            "ARCJ137442" => Some("C")
        }
        test_match_suffix! {
            tuple;
            "INC" => Some("A")
            "SYNC" => Some("A")
            "A, B, C" => Some("A")
            "BASIC" => Some("A")
            "Objective-C" => Some("A")
        }
    }
}
