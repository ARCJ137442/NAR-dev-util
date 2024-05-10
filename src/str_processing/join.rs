//! 辅助各种「字符串join」的方法
//! * 🎯用于各种定制的字符串join方式

use crate::{catch_flow, push_str, AsStrRef};

/// 拼接字串到指定目标
/// * 🎯将字符串集中拼接到一个「目标字串」中，中途不创建任何辅助字符串
/// * 🎯用于替代【会创建[`String`]对象】的[`std::slice::Join::join`]方法
///   * ✨在对其它字串使用类似`join`的方式添加数组元素时，享受**零对象创建**的性能提升
/// * 📝对于兼容[`String`]和[`str`]两种类型
/// * 📝相当于对上边[`AsStrRef`]的展示
///
/// ! [`std::slice::Join`]特征不稳定，参见<https://github.com/rust-lang/rust/issues/27747>
pub fn join_to(out: &mut String, iter: impl Iterator<Item = impl AsStrRef>, sep: impl AsStrRef) {
    // 简单的`join实现
    let mut is_first = true;
    for s in iter {
        // 添加分隔符
        match is_first {
            true => is_first = false,
            false => out.push_str(sep.as_str_ref()),
        }
        // 添加元素
        out.push_str(s.as_str_ref());
    }
}

/// 拼接字符串到新字串
/// * 🎯类似[`join_to`]，但会创建新字串
/// * 🚩基于[`catch_flow`]实现
pub fn join_to_new(iter: impl Iterator<Item = impl AsStrRef>, sep: impl AsStrRef) -> String {
    catch_flow!(join_to; iter, sep)
}

/// 拼接字串到指定目标，但在每次添加时添加多个分隔符
/// * 🎯将字符串集中拼接到一个「目标字串」中，中途不创建任何辅助字符串
/// * 🎯用于「一个条目-多个分隔符-另一个条目」
///   * 📄如：持有","和" "，需要依次添加，但又不想创建`String::from(", ")`的时候
///   * ✨在对其它字串使用类似`join`的方式添加数组元素时，享受**零对象创建**的性能提升
/// * 📝对于兼容[`String`]和[`str`]两种类型
/// * 📝相当于对上边[`AsStrRef`]的展示
///
/// ! [`std::slice::Join`]特征不稳定，参见<https://github.com/rust-lang/rust/issues/27747>
pub fn join_to_multi(
    out: &mut String,
    iter: impl Iterator<Item = impl AsStrRef>,
    separators: &[impl AsStrRef],
) {
    // 简单的`join实现
    let mut is_first = true;
    for s in iter {
        // 添加分隔符
        match is_first {
            true => is_first = false,
            false => {
                for sep in separators {
                    push_str!(out; sep.as_str_ref());
                }
            }
        }
        // 添加元素
        out.push_str(s.as_str_ref());
    }
}

/// 拼接字符串到新字串/多个分隔符
/// * 🎯类似[`join_to_multi`]，但会创建新字串
/// * 🚩基于[`catch_flow`]实现
pub fn join_to_multi_new(
    iter: impl Iterator<Item = impl AsStrRef>,
    sep: &[impl AsStrRef],
) -> String {
    catch_flow!(join_to_multi; iter, sep)
}

/// 工具函数/有内容时前缀分隔符
/// * 🎯最初用于「多个用空格分隔的条目」中「若其中有空字串，就无需连续空格」的情况
/// * 关键在「避免无用分隔符」
pub fn add_space_if_necessary_and_flush_buffer(
    out: &mut String,
    buffer: &mut String,
    separator: impl AsStrRef,
) {
    match buffer.is_empty() {
        // 空⇒不做动作
        true => {}
        // 非空⇒预置分隔符，推送并清空
        false => {
            push_str!(out; separator.as_str_ref(), buffer);
            buffer.clear();
        }
    }
}

/// 工具函数/用分隔符拼接字符串，且当元素为空时避免连续分隔符
/// * 🎯最初用于「多个用空格分隔的条目」中「若其中有空字串，就无需连续空格」的情况
/// * 📌实际上是[`add_space_if_necessary_and_flush_buffer`]的另一种形式
///
/// # Example
/// ```rust
/// use nar_dev_utils::join_lest_multiple_separators;
/// let mut s = String::new();
/// join_lest_multiple_separators(&mut s, vec!["a", "", "b", "c", "", "d"].into_iter(), ",");
/// assert_eq!(s, "a,b,c,d");
/// ```
pub fn join_lest_multiple_separators<S>(
    out: &mut String,
    mut elements: impl Iterator<Item = S>,
    separator: impl AsStrRef,
) where
    S: AsStrRef,
{
    // 先加入第一个元素
    match elements.next() {
        // 有元素⇒直接加入
        Some(s) => out.push_str(s.as_str_ref()),
        // 无元素⇒直接返回
        None => return,
    };
    // 其后「先考虑分隔，再添加元素」
    for element in elements {
        match element.as_str_ref().is_empty() {
            // 空字串⇒没必要添加
            true => continue,
            // 非空字串⇒连同分隔符一并添加
            false => push_str!(out; separator.as_str_ref(), element.as_str_ref()),
        }
    }
}

/// 为迭代器实现`join`系列方法
/// * 🎯尝试补全「只有数组能被`join`」的缺陷
pub trait JoinTo {
    /// 将字串集中拼接到一个「目标字串」中，中途不创建任何辅助字符串
    /// * 📌类似JavaScript的`Array.join()`方法
    /// * 📄参见全局函数[`join_to`]
    fn join_to<S>(self, out: &mut String, sep: impl AsStrRef)
    where
        Self: Iterator<Item = S> + Sized,
        S: AsStrRef,
    {
        join_to(out, self, sep)
    }

    /// 将字串集中拼接到一个新字串中
    /// * 📌类似JavaScript的`Array.join()`方法
    /// * 📄参见全局函数[`join_to`]
    fn join_to_new<S>(self, sep: impl AsStrRef) -> String
    where
        Self: Iterator<Item = S> + Sized,
        S: AsStrRef,
    {
        join_to_new(self, sep)
    }

    /// 将字串集中拼接到一个「目标字串」中，使用多个分隔符，中途不创建任何辅助字符串
    /// * 📄参见全局函数[`join_to_multi`]
    fn join_to_multi<S>(self, out: &mut String, sep: &[impl AsStrRef])
    where
        Self: Iterator<Item = S> + Sized,
        S: AsStrRef,
    {
        join_to_multi(out, self, sep)
    }

    /// 将字串集中拼接到一个新字串中，使用多个分隔符
    /// * 📄参见全局函数[`join_to_multi`]
    fn join_to_multi_new<S>(self, sep: &[impl AsStrRef]) -> String
    where
        Self: Iterator<Item = S> + Sized,
        S: AsStrRef,
    {
        join_to_multi_new(self, sep)
    }
}

impl<T> JoinTo for T {}

/// 专门实现的 `join!` 宏
mod macro_join_to {
    /// 特制的「加入」方法
    /// * 🎯为[`String`]提供比`+=`与[`push`](String::push)
    pub trait MacroJoinable<Suffix> {
        fn join_to(self, suffix: Suffix);
    }

    impl MacroJoinable<&str> for &mut String {
        fn join_to(self, suffix: &str) {
            self.push_str(suffix);
        }
    }

    impl MacroJoinable<&String> for &mut String {
        fn join_to(self, suffix: &String) {
            self.push_str(suffix);
        }
    }

    impl MacroJoinable<String> for &mut String {
        fn join_to(self, suffix: String) {
            self.push_str(&suffix); // ! 既然要消耗所有权，那就加个引用咯
        }
    }

    impl MacroJoinable<char> for &mut String {
        fn join_to(self, suffix: char) {
            self.push(suffix);
        }
    }

    // ! ❌【2024-05-10 21:54:36】放弃「先实现可变，再对『可变』批量实现『不可变』」的思路：生命周期问题
    //   ! `(&mut self).join_to(suffix)`不起作用：`(&mut self)`「不在生命周期内」「仍然一直引用」
    // * ✅现在通过特制的「自动转所有权」语法，实现「表达式体」「语句体」的兼备

    /// # 流式拼接
    /// * 🎯以「流式处理」的办法，方便且高性能地拼接各种表达式
    /// * 🚩基于特征[`MacroJoinable`]作动态分派，以实现高性能
    /// * ⚡对字符调用`push`，对`&str`、`&String`调用`push_str`
    /// * 📌除了`format!`产生额外字符串的开销外，基本与「不断调用`push`、`push_str`、`+=`」一致
    /// * ✨支持在拼接过程中插入更复杂的控制结构，如`if`、`while`、`for`
    ///
    /// ## 测试用例
    ///
    /// ```rust
    /// use nar_dev_utils::join;
    /// let mut s = "text: ".to_string();
    /// join!(
    ///     &mut s // "text: "
    ///     => {# 1} // 1
    ///     => ' ' // 【空格】
    ///     => {# "1" ; ?} // "1"（格式化）
    ///     => " " // 【空格】
    ///     => {# [1, 2, 3] ; ?} // 普通格式化数组（数组本身不支持`Display`）
    ///     => '\n' // 【换行】
    ///     => {# (1, 2, (3, 4)) ; #?} // 带换行缩进的格式化
    /// );
    /// let mut a = 0;
    /// let s2 = join!(
    ///     => {# 12 ; 0>4} // 0012
    ///     => " " // 【空格】
    ///     => {# a} while {a += 1; a == 1} // 13（a=1，条件满足，随后跳到a=2）
    ///     => {# " {a}" in} // 2（多加个`in`代表在格式化）
    ///     => ' ' // 【空格】
    ///     => {# "0x{:X}" in 0xabc} if let (_, 42) = ("", 42) // 0xABC（if let 条件）
    ///     => {# " 0b{:b} 0o{:o}_u64" in 0b101, 0o33653337357_u64} // 0o33653337357_u64
    ///     => "13" while let Some(1) = Some(a) // 无（a=2，while let条件不满足）
    ///     => &" ".to_string() // 【空格】
    ///     => {# i} for i in 0..=9 // 0 1 2 3 4 5 6 7 8 9（for循环）
    /// );
    /// assert_eq!(
    ///     s,
    ///     "text: 1 \"1\" [1, 2, 3]\n(\n    1,\n    2,\n    (\n        3,\n        4,\n    ),\n)"
    /// );
    /// assert_eq!(s2, "0012 1 2 0xABC 0b101 0o33653337357_u64 0123456789");
    /// ```
    #[macro_export]
    macro_rules! join {
        // `{# }`格式化
        (@EX {# $ex:expr}) => {
            format!("{}", $ex)
        };
        // `{# ;?#}`格式化
        (@EX {# $ex:expr ; $($fmt:tt)*}) => {
            format!(concat!("{:", stringify!($($fmt)*), "}"), $ex)
        };
        // `{# "0x{:X}" in $ex}`格式化
        (@EX {# $fmt:literal in $($ex:tt)*}) => {
            format!($fmt, $($ex)*)
        };
        // 兜底表达式
        (@EX $ex:expr) => {
            $ex
        };
        // `=> $string`代表「传入所有权，传出所有权」的情形
        // 传所有权/主入口
        (
            => $string:tt
            $( => $($tail:tt)*)?
        ) => {
            {
                // 捕获值（直接使用新字串）
                let mut string_mut = $crate::join!(@EX $string);
                // 用其可变引用继续处理
                $crate::join!(&mut string_mut $( => $($tail)*)?);
                // 返回所捕获值
                string_mut
            }
        };
        // 传所有权/表达式简写
        (
            => $string:expr
            $( => $($tail:tt)*)?
        ) => {
            $crate::join!(
                => ($string) // * 🚩直接用个括号包裹，以代表其为表达式
                $( => $($tail)*)?
            )
        };
        // 中间过程/统一语法 `(表达式)` `{#格式化}`
        (
            $string:expr
            => $ex:tt
            $( => $($tail:tt)*)?
        ) => {
            // 处理追加，基于`MacroJoinable`特征
            $crate::MacroJoinable::join_to(
                $string,
                $crate::join!(@EX $ex) // 使用不可变引用
            );
            $crate::join!($string $( => $($tail)*)?);
        };
        // 中间过程/条件`if`语法
        (
            $string:expr
            => $ex:tt if $condition:expr
            $( => $($tail:tt)*)?
        ) => {
            if $condition {
                $crate::MacroJoinable::join_to(
                    $string,
                    $crate::join!(@EX $ex)
                );
            }
            $crate::join!($string $( => $($tail)*)?);
        };
        // 中间过程/条件`if let`语法
        (
            $string:expr
            => $ex:tt if let $pattern:pat = $condition:expr
            $( => $($tail:tt)*)?
        ) => {
            if let $pattern = $condition {
                $crate::MacroJoinable::join_to(
                    $string,
                    $crate::join!(@EX $ex)
                );
            }
            $crate::join!($string $( => $($tail)*)?);
        };
        // 中间过程/循环`while`语法
        (
            $string:expr
            => $ex:tt while $condition:expr
            $( => $($tail:tt)*)?
        ) => {
            while $condition {
                $crate::MacroJoinable::join_to(
                    $string,
                    $crate::join!(@EX $ex)
                );
            }
            $crate::join!($string $( => $($tail)*)?);
        };
        // 中间过程/循环`while let`语法
        (
            $string:expr
            => $ex:tt while let $pattern:pat = $condition:expr
            $( => $($tail:tt)*)?
        ) => {
            while let $pattern = $condition {
                $crate::MacroJoinable::join_to(
                    $string,
                    $crate::join!(@EX $ex)
                );
            }
            $crate::join!($string $( => $($tail)*)?);
        };
        // 中间过程/循环`for`语法
        (
            $string:expr
            => $ex:tt for $pattern:pat in $iter:expr
            $( => $($tail:tt)*)?
        ) => {
            for $pattern in $iter {
                $crate::MacroJoinable::join_to(
                    $string,
                    $crate::join!(@EX $ex)
                );
            }
            $crate::join!($string $( => $($tail)*)?);
        };
        // 中间过程/表达式简写（兜底）
        (
            $string:expr
            => $ex:expr
            $( => $($tail:tt)*)?
        ) => {
            $crate::join!(
                $string
                => ($ex) // ! 圆括弧括起，转发
                $( => $($tail)*)?
            );
        };
        // 兜底
        ( $string:expr ) => {};
    }
}

pub use macro_join_to::*;

/// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{asserts, catch_flow};

    #[test]
    fn test_join_to() {
        asserts! {
            // 静态字串
            catch_flow!(join_to; ["a", "b", "c"].iter(), ",") => "a,b,c",
            ["a", "b", "c"].iter().join_to_new(",") => "a,b,c"
            // 动态字串
            catch_flow!(
                join_to;
                [
                    String::from("a"),
                    String::from("b"),
                    String::from("c"),
                    ].iter(),
                    String::from(","),
            ) => "a,b,c"
            //多个字符参数
            catch_flow!(join_to_multi; ["a", "b", "c"].iter(), &[",", " "]) => "a, b, c"
            catch_flow!(join_to_multi; ["a", "b", "c"].iter(), &[",".to_owned(), " ".to_owned()]) => "a, b, c",
            ["a", "b", "c"].iter().join_to_multi_new(&[",", " "]) => "a, b, c"
        }
    }

    #[test]
    fn test_add_space_if_necessary_and_flush_buffer() {
        asserts! {
            // 缓冲区有元素⇒加上分隔符
            {
                let mut s = String::from("A");
                let mut buffer = String::from("B");
                add_space_if_necessary_and_flush_buffer(&mut s, &mut buffer, ",");
                (s, buffer)
            } => ("A,B".into(), "".into())
            // 缓冲区没元素⇒不加分隔符
            {
                let mut s = String::from("A");
                let mut buffer = String::from("");
                add_space_if_necessary_and_flush_buffer(&mut s, &mut buffer, ",");
                (s, buffer)
            } => ("A".into(), "".into())
        }
    }

    #[test]
    fn test_join_lest_multiple_separators() {
        asserts! {
            // 几个都有的情况
            catch_flow!(
                join_lest_multiple_separators;
                ["A", "B", "C"].iter(),
                ", "
            ) => "A, B, C"
            // 有些没有的情况
            catch_flow!(
                join_lest_multiple_separators;
                ["A", "B", "", "C"].iter(),
                ", "
            ) => "A, B, C"
        }
    }
}
