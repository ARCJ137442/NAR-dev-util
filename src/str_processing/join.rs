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
