//! 「前后缀匹配/词缀匹配 字典」
//! * 🎯用于统一【前后无关】的「前缀匹配字典」与「后缀匹配字典」
//!   * 🚩实际的核心逻辑就是「按照『长度降序』遍历词缀」
//! * 🎯用于像「陈述系词」「复合词项连接词」这样「前后无关」的词缀匹配
//!   * 📄case: `"-->"`既可以作为前缀匹配，也可以作为后缀匹配
use super::traits::*;

/// 统一定义「词缀」类型
/// * 🎯用以替代四处使用的[`String`]
/// * 🎯用以同时指代[`Prefix`]、[`Suffix`]两者
type XFix = String;

/// 前后缀匹配字典 / 词缀匹配字典
/// * 🚩具体逻辑：
///   * 维护一个有一定顺序、不重复的[`XFix`]数组
///   * 在匹配时【按长度倒序】迭代出前缀
/// * 📌【2024-03-17 11:13:12】此处使用`XFix`指代`Prefix`与`Suffix`两者
#[derive(Debug, Clone, Default)]
pub struct XFixMatchDict {
    x_fixes: Vec<XFix>,
}

/// 原「前缀匹配字典」
/// * 🚩现在统一并入「词缀匹配字典」
#[doc(alias = "XFixMatchDict")]
pub type PrefixMatchDict = XFixMatchDict;

/// 原「后缀匹配字典」
/// * 🚩现在统一并入「词缀匹配字典」
#[doc(alias = "XFixMatchDict")]
pub type SuffixMatchDict = XFixMatchDict;

impl PrefixMatchDict {
    /// 构造函数
    /// * 支持从任何「元素为『可转换为字符串』的可迭代对象」中转换
    pub fn new(prefixes: impl IntoIterator<Item = impl Into<XFix>>) -> Self {
        // ? 或许也可以「先新建空值，然后逐个添加」来实现，复杂度 ∑ 1 log 1 ~ n log n
        Self {
            x_fixes: prefixes
                .into_iter()
                .map(|into_s| into_s.into())
                .collect::<Vec<_>>(),
        }
    }

    /// （前后缀无关）判断「是否已有一个词缀」
    /// * 📌直接使用自身的「搜索」功能
    pub fn has(&self, x_fix: &XFix) -> bool {
        // * 🚩查找「ok」证明「能找到」
        self.search(x_fix).is_ok()
    }

    /// （前后缀无关）插入一个词缀
    /// * 🚩调用经分派的「查找」方法
    /// * 🚩返回「是否成功插入」
    pub fn insert(&mut self, x_fix: XFix) {
        match self.search(&x_fix) {
            // 已有⇒跳过
            Ok(..) => {}
            // 未找到
            Err(index) => {
                self.x_fixes.insert(index, x_fix);
            }
        }
    }

    /// （前后缀无关）以特殊顺序迭代「词缀」
    /// * 🎯统一「前缀匹配」与「后缀匹配」的迭代逻辑
    /// * 🚩总是按照「字典顺序」倒序遍历：**长度从长到短**
    pub fn iter_x_fixes<'a>(&'a self) -> impl Iterator<Item = &'a XFix> + 'a
    where
        XFix: 'a,
    {
        // ! 这里必须倒过来，从长到短匹配
        self.x_fixes.iter().rev()
    }

    /// 搜索 | 使用二分查找
    /// * 🎯构造可方便替换的「查找」逻辑
    /// * 🚩找到⇒位置，没找到⇒应该插入的位置
    #[cfg(feature = "algorithms")]
    #[inline(always)]
    pub fn search(&self, x_fix: &XFix) -> Result<usize, usize> {
        use crate::binary_search;
        binary_search(&self.x_fixes, x_fix)
    }

    /// 搜索 | 使用线性查找
    /// * 🎯构造可方便替换的「查找」逻辑
    /// * 🚩找到⇒位置，没找到⇒应该插入的位置
    #[cfg(not(feature = "algorithms"))]
    #[inline(always)]
    pub fn search(&self, x_fix: &XFix) -> Result<usize, usize> {
        // 线性匹配
        use std::cmp::Ordering;
        for (i, existed) in self.x_fixes.iter().enumerate() {
            match x_fix.cmp(existed) {
                // =
                Ordering::Equal => return Ok(i),
                // < | 确保匹配到「第一个比自己大的」
                Ordering::Less => return Err(i),
                // >
                Ordering::Greater => (),
            }
        }
        // 否则插入末尾
        Err(self.x_fixes.len())
    }
}

/// 快速生成「词缀匹配字典」
#[macro_export]
macro_rules! x_fix_match_dict {
    // 转换其中的值 | 静态字串⇒动态字串 自动`into`
    (@VALUE $v:literal) => {
        $v.into()
    };
    // 转换其中的值 | 表达式⇒直接加入
    (@VALUE $v:expr) => {
        $v
    };
    // 统一的表 | 自面量也是一种表达式
    [$($item:expr $(,)?)*] => {{
        let mut d = PrefixMatchDict::default();
        $(
            d.insert(x_fix_match_dict!(@VALUE $item));
        )*
        d
    }};
}

/// 兼容性重定向「前缀匹配字典」
/// * 📝使用修饰属性`local_inner_macros`一并导出里边用到的宏
#[macro_export(local_inner_macros)]
macro_rules! prefix_match_dict {
    ( $($anything:tt)* ) => {
        $crate::x_fix_match_dict!($($anything)*)
    };
}

/// 兼容性重定向「后缀匹配字典」
/// * 📝使用修饰属性`local_inner_macros`一并导出里边用到的宏
#[macro_export(local_inner_macros)]
macro_rules! suffix_match_dict {
    ( $($anything:tt)* ) => {
        $crate::x_fix_match_dict!($($anything)*)
    };
}

/// 实现「前缀匹配」
impl PrefixMatch<XFix> for PrefixMatchDict {
    // 前缀就是它本身
    fn get_prefix_from_term(term: &XFix) -> &PrefixStr {
        term
    }

    // 直接重定向
    fn prefix_terms<'a>(&'a self) -> impl Iterator<Item = &'a XFix> + 'a
    where
        XFix: 'a,
    {
        self.iter_x_fixes()
    }
}

/// 实现「后缀匹配」
impl SuffixMatch<XFix> for SuffixMatchDict {
    // fn new_suffix_term(suffix: Suffix, associated: XFix) -> XFix {}
    // 后缀就是它本身
    fn get_suffix_from_term(term: &XFix) -> &SuffixStr {
        term
    }

    // 直接重定向
    fn suffix_terms<'a>(&'a self) -> impl Iterator<Item = &'a XFix> + 'a
    where
        XFix: 'a,
    {
        self.iter_x_fixes()
    }
}

/// 单元测试/前缀匹配
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{asserts, show};

    /// 实用宏 @ 用于生成「批量词缀匹配」
    #[macro_export] // ! 虽然导出了，但因为`#[cfg(test)]`还是不会污染全局环境
    macro_rules! test_match_x_fix {
        {
            $f_name:ident;
            $post_process:expr;
            $d:expr;
            // 待匹配的字符串自面量 ⇒ 匹配到的字符串自面量(Option)
            $( $to_match:expr => $expected:expr $(,)?)*
        } => {
            $crate::asserts! {
                $(
                    $d.$f_name($to_match).map($post_process) => $expected
                )*
            }
        };
    }

    /// 实用宏 @ 批量测试前缀匹配
    /// * 📝使用修饰属性`local_inner_macros`一并导出里边用到的宏
    #[macro_export(local_inner_macros)]
    macro_rules! test_match_prefix {
        {
            $($other:tt)*
        } => {
            // 直接内容重定向
            $crate::test_match_x_fix! {
                match_prefix;
                // * 📝↓相比`String::as_str`「先解引用再取引用」更通用（静态动态字串均可）
                |s| &*s.1; // 前缀⇒后缀
                // !    ↑【2024-03-17 15:57:57】暂且还是硬编码的索引
                $($other)*
            }
        };
    }

    // 实用宏 @ 批量测试后缀匹配
    /// * 📝使用修饰属性`local_inner_macros`一并导出里边用到的宏
    #[macro_export(local_inner_macros)]
    macro_rules! test_match_suffix {
        {
            $($other:tt)*
        } => {
            // 直接内容重定向
            $crate::test_match_x_fix! {
                match_suffix;
                // * 📝↓相比`String::as_str`「先解引用再取引用」更通用（静态动态字串均可）
                |s| &*s.0; // 后缀⇒前缀
                // !    ↑【2024-03-17 15:57:57】暂且还是硬编码的索引
                $($other)*
            }
        };
    }

    /// 测试/前缀匹配
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
                        $d.match_prefix($to_match).map(String::as_str) => $expected
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
            r"@, A, B, C" => None
            r"!, A, B, C" => None
            r"`, A, B, C" => None
            r"#, A, B, C" => None
            r"$, A, B, C" => None
            r"%, A, B, C" => None
            r"^, A, B, C" => None
            r"., A, B, C" => None
            r"<, A, B, C" => None
            r">, A, B, C" => None
            r"?, A, B, C" => None
            r":, A, B, C" => None
            r";, A, B, C" => None
            r"', A, B, C" => None
            r"_, A, B, C" => None
            r"+, A, B, C" => None
            r"=, A, B, C" => None
            r"0, A, B, C" => None
            r"文, A, B, C" => None
            r"🤔, A, B, C" => None
            r"🚩, A, B, C" => None
        }
    }

    /// 测试/后缀匹配
    #[test]
    fn test_suffix_match() {
        // 实用宏
        macro_rules! mpf {
            {
                $d:expr;
                // 待匹配的字符串自面量 ⇒ 匹配到的字符串自面量(Option)
                $( $to_match:expr => $expected:expr $(,)?)*
            } => {
                asserts! {
                    $(
                        $d.match_suffix($to_match).map(String::as_str) => $expected
                    )*
                }
            };
        }
        // 零长字串匹配
        let d = suffix_match_dict!(
            // * 🎯用于时间戳的「空前缀匹配」如`(":|:", "")`
            r":|:"
            r":/:"
            r":\:"
            r":" // * 🎯用于「固定」时间戳 @ ASCII
            r"" // * 🎯用于「固定」时间戳 @ LaTeX/漢文
        );
        show!(&d);
        // 测试后缀匹配
        mpf! {
            d;
            // 长的优先
            r"<A --> B>. :|:" => Some(r":|:")
            r"<A --> B>. :/:" => Some(r":/:")
            r"<A --> B>. :\:" => Some(r":\:")
            r"<A --> B>. :!+137:" => Some(r":")
            // 空字串永远兜底
            r"<A --> B>." => Some("")
            r"「A是B」。" => Some("")
        }
        let d = suffix_match_dict!(
            // 🎯ASCII
            "." "!" "?" "@"
            // 🎯LaTeX
            "." "!" "?" "¿"
            // 🎯漢文
            "。" "！" "？" "；"
        );
        show!(&d);

        // 测试后缀匹配
        mpf! {
            d;
            // 所有枚举情况
            "<A --> B>." => Some(".")
            "<A --> B>!" => Some("!")
            "<A --> B>?" => Some("?")
            "<A --> B>@" => Some("@")
            r"\left<A \rightarrow  B\right>." => Some(".")
            r"\left<A \rightarrow  B\right>!" => Some("!")
            r"\left<A \rightarrow  B\right>?" => Some("?")
            r"\left<A \rightarrow  B\right>¿" => Some("¿")
            r"「A是B」。" => Some("。")
            r"「A是B」！" => Some("！")
            r"「A是B」？" => Some("？")
            r"「A是B」；" => Some("；")
            // 无效情况
            ""            => None // 空字串必定匹配不了
            r"<A --> B>`" => None
            r"<A --> B>#" => None
            r"<A --> B>$" => None
            r"<A --> B>%" => None
            r"<A --> B>^" => None
            r"<A --> B>&" => None
            r"<A --> B>*" => None
            r"<A --> B>(" => None
            r"<A --> B>)" => None
            r"<A --> B>-" => None
            r"<A --> B>_" => None
            r"<A --> B>+" => None
            r"<A --> B>=" => None
            r"<A --> B><" => None
            r"<A --> B>>" => None
            r"<A --> B>," => None
            r"<A --> B>:" => None
            r"<A --> B>;" => None
            r"<A --> B>'" => None
            r"<A --> B>/" => None
            r"<A --> B>\" => None
            r"<A --> B>0" => None
            r"<A --> B>文" => None
            r"<A --> B>🤔" => None
            r"<A --> B>🚩" => None
        }
    }
}
