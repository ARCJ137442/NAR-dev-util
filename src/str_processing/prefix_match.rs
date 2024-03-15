//! 与「前缀匹配」有关的工具结构与算法
//! * 🎯最初用于字符串parser

use crate::{binary_search, binary_search_by};

/// 前缀匹配（抽象特征）
/// * 🎯用于存储前缀，封装如下两个逻辑
///   * 前缀匹配→返回被匹配项：用于匹配如「原子词项前缀」的一次性匹配
///   * 前缀匹配→返回前缀、后缀：用于匹配如「不同自定义括弧」的「配对性匹配」
///     * 🎯可以省去另一个字典映射
/// * 📌其中的前缀总是[`String`]类型
///   * 并且是**不重复**的
/// * 🎯解决「短的先匹配到截断了，长的因此无法被匹配到」的问题
/// * 🚩此处不采取「条目与前缀分离」的做法
///   * 「分离式条目」可以用`条目 = (前缀, 其它内容)`模拟
pub trait PrefixMatch<PrefixTerm> {
    /// 【抽象】用于从一个「前缀条目」中获取「前缀」（字符串）
    fn get_prefix_from_term<'a>(&'a self, term: &'a PrefixTerm) -> &'a String;

    /// 【抽象】插入一个「前缀条目」
    /// * 🎯通用于「单纯前缀匹配」与「配对前缀匹配」
    fn insert(&mut self, term: PrefixTerm);

    /// 【抽象】迭代「前缀」和「前缀条目」
    /// * 🎯用于后续匹配
    /// * ⚠️因此需要【倒序】匹配：长的字串先来，然后是短的
    ///   * 避免"&"比"&&"优先
    fn prefixes_and_items<'a>(&'a self) -> impl Iterator<Item = &'a PrefixTerm> + 'a
    where
        PrefixTerm: 'a;

    /// 开启前缀匹配
    /// * 🎯封装「前缀匹配」逻辑，通用于「单纯前缀匹配」与「配对前缀匹配」
    /// * 🚩迭代、扫描、匹配
    ///   * 1. 从一个字符串开始
    ///   * 2. 然后扫描自身所有前缀（字串从长到短）
    ///   * 3. 最后（若成功）返回匹配到的前缀所对应的「前缀条目」
    fn match_prefix(&self, to_match: &str) -> Option<&PrefixTerm> {
        // * ↓非迭代器版本
        // for (prefix, term) in self.prefixes_and_items() {
        //     if to_match.starts_with(prefix) {
        //         return Some(term);
        //     }
        // }
        // None
        // ✅迭代器版本
        self.prefixes_and_items()
            .find(|&term| to_match.starts_with(self.get_prefix_from_term(term)))
    }
}

/// 前缀匹配字典
/// * 🚩具体逻辑：
///   * 维护一个有一定顺序、不重复的[`String`]数组
#[derive(Debug, Clone, Default)]
pub struct PrefixMatchDict {
    prefixes: Vec<String>,
}

impl PrefixMatchDict {
    /// 构造函数
    /// * 支持从任何「元素为『可转换为字符串』的可迭代对象」中转换
    pub fn new(prefixes: impl IntoIterator<Item = impl Into<String>>) -> Self {
        // ? 或许也可以「先新建空值，然后逐个添加」来实现，复杂度 ∑ 1 log 1 ~ n log n
        Self {
            prefixes: prefixes
                .into_iter()
                .map(|into_s| into_s.into())
                .collect::<Vec<_>>(),
        }
    }
}

#[macro_export]
macro_rules! prefix_match_dict {
    // 转换其中的值 | 静态字串⇒动态字串 自动`into`
    (@value $v:literal) => {
        $v.into()
    };
    // 转换其中的值 | 表达式⇒直接加入
    (@value $v:expr) => {
        $v
    };
    // 统一的表 | 自面量也是一种表达式
    [$($item:expr $(,)?)*] => {{
        let mut d = PrefixMatchDict::default();
        $(
            d.insert(prefix_match_dict!(@value $item));
        )*
        d
    }};
}

impl PrefixMatch<String> for PrefixMatchDict {
    // 前缀就是它本身
    fn get_prefix_from_term<'a>(&'a self, term: &'a String) -> &'a String {
        term
    }

    // 🚩使用二分查找搜寻
    fn insert(&mut self, prefix: String) {
        match binary_search(&self.prefixes, &prefix) {
            // 已有⇒跳过
            Ok(..) => {}
            // 未找到
            Err(index) => {
                self.prefixes.insert(index, prefix);
            }
        }
    }

    fn prefixes_and_items<'a>(&'a self) -> impl Iterator<Item = &'a String> + 'a
    where
        String: 'a,
    {
        // ! 这里必须倒过来，从长到短匹配
        self.prefixes.iter().rev()
    }
}

/// 前缀匹配字典
/// * 🚩具体逻辑：
///   * 维护一个有一定顺序、不重复的[`String`]数组
#[derive(Debug, Clone, Default)]
pub struct PrefixMatchDictPair<T> {
    prefixes: Vec<(String, T)>,
}

impl<T> PrefixMatchDictPair<T> {
    /// 构造函数
    /// * 支持从任何「元素为『可转换为字符串』的可迭代对象」中转换
    pub fn new(prefixes: impl IntoIterator<Item = (impl Into<String>, T)>) -> Self {
        // ? 或许也可以「先新建空值，然后逐个添加」来实现，复杂度 ∑ 1 log 1 ~ n log n
        Self {
            prefixes: prefixes
                .into_iter()
                .map(|(into_s, t)| (into_s.into(), t))
                .collect::<Vec<_>>(),
        }
    }
}

#[macro_export]
macro_rules! prefix_match_dict_pair {
    // 转换其中的值 | 静态字串⇒动态字串 自动`into`
    (@value $v:literal) => {
        $v.into()
    };
    // 转换其中的值 | 表达式⇒直接加入
    (@value $v:expr) => {
        $v
    };
    // 统一的表 | 自面量也是一种表达式
    [$($prefix:expr => $item:expr $(,)?)*] => {{
        let mut d = PrefixMatchDictPair::default();
        $(
            d.insert((
                prefix_match_dict_pair!(@value $prefix),
                prefix_match_dict_pair!(@value $item),
            ));
        )*
        d
    }};
}

impl<T> PrefixMatch<(String, T)> for PrefixMatchDictPair<T> {
    fn get_prefix_from_term<'a>(&'a self, term: &'a (String, T)) -> &'a String {
        &term.0
    }
    /// 插入一个字符串元素
    fn insert(&mut self, term: (String, T)) {
        match binary_search_by(&self.prefixes, &term, |existed, new| existed.0.cmp(&new.0)) {
            // 已有⇒跳过
            Ok(..) => {}
            // 未找到
            Err(index) => {
                self.prefixes.insert(index, term);
            }
        }
    }

    fn prefixes_and_items<'a>(&'a self) -> impl Iterator<Item = &'a (String, T)> + 'a
    where
        (String, T): 'a,
    {
        self.prefixes.iter().rev()
    }
}

/// 单元测试/前缀匹配
#[cfg(test)]
mod tests {
    use crate::{asserts, show};

    use super::*;

    #[test]
    fn test_prefix_match() {
        // 实用宏
        macro_rules! mpf {
            {
                $d:expr;
                // 待匹配的字符串自面量 ⇒ 匹配到的字符串自面量(Option)
                $( $to_match:expr => $expected:expr $(,)?)*
            } => {
                asserts! {
                    $(
                        $d.match_prefix($to_match).map(|s|s.as_str()) => $expected
                    )*
                }
            };
        }
        // 零长字串匹配
        let d = prefix_match_dict!(
            ""
            "$" "#" "?"
            "+"
            "^"
        );
        show!(&d);
        // 测试前缀匹配
        mpf! {
            d;
            "$independent" => Some("$")
            "#dependent" => Some("#")
            "?query" => Some("?")
            "+137" => Some("+")
            "^operator" => Some("^")
            // 空字串永远兜底
            "word" => Some("")
        }
        let d = prefix_match_dict!(
            "&" "|" "-" "~"
            "*" "/" "\\"
            "&&" "||" "--"
            "&/" "&|"
        );
        show!(&d);

        // 测试前缀匹配
        mpf! {
            d;
            // 长的优先
            "&&, A, B, C" => Some("&&")
            "&/, A, B, C" => Some("&/")
            "&|, A, B, C" => Some("&|")
            "&, A, B, C" => Some("&")
            "||, A, B, C" => Some("||")
            "|, A, B, C" => Some("|")
            "--, A" => Some("--")
            "-, A, B" => Some("-")
            // 其它匹配的情况
            r"~, A, B" => Some(r"~")
            r"*, A, B, C" => Some(r"*")
            r"/, A, B, C" => Some(r"/")
            r"\, A, B, C" => Some(r"\")
            // 无效情况
            "" => None // 空字串必定匹配不了
            "@, A, B, C" => None
            "!, A, B, C" => None
            "`, A, B, C" => None
            "#, A, B, C" => None
            "$, A, B, C" => None
            "%, A, B, C" => None
            "^, A, B, C" => None
            "., A, B, C" => None
            "<, A, B, C" => None
            ">, A, B, C" => None
            "?, A, B, C" => None
            ":, A, B, C" => None
            ";, A, B, C" => None
            "', A, B, C" => None
           "\", A, B, C" => None
            "_, A, B, C" => None
            "+, A, B, C" => None
            "=, A, B, C" => None
            "0, A, B, C" => None
            "文, A, B, C" => None
            "🤔, A, B, C" => None
            "🚩, A, B, C" => None
        }
    }

    #[test]
    fn test_prefix_match_pairs() {
        // 实用宏
        macro_rules! mpf {
            {
                $d:expr;
                // 待匹配的字符串自面量 ⇒ 匹配到的字符串自面量(Option)
                $( $to_match:expr => $expected:expr $(,)?)*
            } => {
                asserts! {
                    $(
                        $d.match_prefix($to_match).map(|s|s.1.as_str()) => $expected
                    )*
                }
            };
        }
        let d: PrefixMatchDictPair<String> = prefix_match_dict_pair!(
            "(" => ")"
            "[" => "]"
            "{" => "}"
            "<" => ">"
        );
        show!(&d);
        // 测试前缀匹配
        mpf! {
            d;
            // 范围内情况
            r"(A, B, C)" => Some(")")
            r"[A, B, C]" => Some("]")
            r"{A, B, C}" => Some("}")
            r"<A, B, C>" => Some(">")
            // 无效情况
            "word" => None
        }
    }
}
