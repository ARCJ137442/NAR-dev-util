//! 有关「字符串带所有权拆分」的模块
//! * 🎯提供【零额外空间开销】的字符串拆分功能

/// 用于补足[`Pattern`](crate::str::Pattern)不稳定性的短板
/// * 📌主要功能：一次查找返还两个量
///   * 📍首个字符的索引位置
///   * 📍整个图式的[`u8`]长度
/// * 🚩【2024-08-17 21:45:44】目前需要[`Copy`]实属「保存在结构体中」的无奈
///   * ⚠️对于`&[char]`无法确定「选中的是哪个[`char`]」因此导致「无法确认选中的图式长度」
/// * ✨后续可扩展，或直接基于稳定后的[`Pattern`](crate::str::Pattern)特征加入
pub trait PatternWithLen {
    fn find_with_len(&self, haystack: &str) -> Option<(usize, usize)>;
}

/// 统一对闭包实现
/// * 📌【2024-08-17 21:57:27】此处基于可copy的[`Fn`]
impl<F: Fn(char) -> bool> PatternWithLen for F {
    fn find_with_len(&self, haystack: &str) -> Option<(usize, usize)> {
        haystack.find(self).map(|i| (i, haystack[i..=i].len()))
    }
}

impl PatternWithLen for char {
    fn find_with_len(&self, haystack: &str) -> Option<(usize, usize)> {
        haystack.find(*self).map(|i| (i, self.len_utf8()))
    }
}

impl PatternWithLen for &str {
    fn find_with_len(&self, haystack: &str) -> Option<(usize, usize)> {
        haystack.find(self).map(|i| (i, self.len()))
    }
}

/// 用于「根据指定字符拆分字符串」的迭代器
/// * 🔗参考：<https://www.reddit.com/r/rust/comments/qxcp1w/why_cant_you_split_a_string>
#[derive(Debug, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct IterSplitCharOwned<Pattern: PatternWithLen> {
    /// 剩余的字符串
    residual: Option<String>,
    /// 分隔用图式（可拷贝）
    pattern: Pattern,
}

impl<Pattern: PatternWithLen> Iterator for IterSplitCharOwned<Pattern> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let residual = self.residual.as_mut()?;
        // 寻找下一个换行符
        let mut new_residual = match self.pattern.find_with_len(residual) {
            Some((index_begin_of_delim, len_delim)) => {
                let index_end_of_delim = index_begin_of_delim + len_delim;
                let new_residual = residual.split_off(index_end_of_delim);
                residual.truncate(index_begin_of_delim);
                new_residual
            }
            // 没分隔符了⇒返回自身所持有的字符串
            None => return self.residual.take(),
        };
        // 将剩余的字符串移动到 residual 中
        std::mem::swap(residual, &mut new_residual);
        // 获取并返回被截去的字符串
        let splitted_out = new_residual;
        Some(splitted_out)
    }
}

/// 用于「根据指定字符拆分字符串」的迭代器
/// * 🔗参考：<https://www.reddit.com/r/rust/comments/qxcp1w/why_cant_you_split_a_string>
#[derive(Debug, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct LinesOwned {
    /// 剩余的字符串
    residual: Option<String>,
}

impl LinesOwned {
    /// 参照[`core::str::Lines`]内部的`LinesMap`（私有）制作
    /// * 📌返回`(起始索引, 子串长度)`
    /// * 🚩先拿到换行`\n`，然后试着到回头拿回车`\r`
    fn find_delim(s: &str) -> Option<(usize, usize)> {
        const LEN_CR: usize = "\r".len();
        const LEN_LF: usize = "\n".len();
        const LEN_CRLF: usize = "\r\n".len();
        // 先拿到换行符索引
        let lf_index = s.find('\n')?;
        if lf_index >= LEN_CR {
            // 若有可能，尝试拿回车符
            let cr_index = lf_index - LEN_CR;
            // ⚠️此处单凭相减得到的索引，可能不是合法UTF-8位置
            if s.is_char_boundary(cr_index) && s[cr_index..lf_index] == *"\r" {
                // 换行回车
                return Some((cr_index, LEN_CRLF));
            }
        }
        // 不然只有换行
        Some((lf_index, LEN_LF))
    }
}

impl Iterator for LinesOwned {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let residual = self.residual.as_mut()?;
        // 寻找下一个换行符
        let mut new_residual = match Self::find_delim(residual) {
            Some((index_begin_of_delim, len_delim)) => {
                let index_end_of_delim = index_begin_of_delim + len_delim;
                debug_assert!(
                    residual.is_char_boundary(index_end_of_delim),
                    "不会发生：find_delim在{residual:?}中找到的索引{index_begin_of_delim}应该在合法UTF-8位置"
                );
                let new_residual = residual.split_off(index_end_of_delim);
                residual.truncate(index_begin_of_delim);
                new_residual
            }
            // 没分隔符了⇒返回自身所持有的字符串
            // * ✅应对"abc\n"的情况也不会「当作两行」：
            //   * 当「拆分出"abc"」之后，`residual`就会被置空
            None => {
                return self.residual.take();
                // return match self.residual.take() {
                //     Some(x) if x.is_empty() => None,
                //     x => x,
                // };
            }
        };
        // 将剩余的字符串移动到 residual 中
        std::mem::swap(residual, &mut new_residual);
        if residual.is_empty() {
            self.residual = None;
        }
        // 获取并返回被截去的字符串
        let splitted_out = new_residual;
        Some(splitted_out)
    }
}

/// 通用的「带所有权拆分」特征
/// * 🎯对占用空间较大的字符串 无拷贝拆分
///   * 📄超长JSON文本
/// * ⚠️【2024-08-17 21:25:00】因[`Pattern`](std::str::pattern::Pattern)尚未稳定，此处仅使用`char`
pub trait SplitOwned: Sized {
    /// 以某个固定的字符分隔字符串
    /// * 🎯[`str::split`]的带所有权版本（不完整）
    fn split_owned<Pattern: PatternWithLen>(self, pat: Pattern) -> impl Iterator<Item = String>;

    /// 带所有权地拆分字符串的行
    /// * 🎯无空间开销地拆分字符串
    ///   * 📄场景：一个数十Kb级大小的JSON文本要拆成两行，需要尽可能避免内容复制
    /// * ⚡可避免拷贝字符串
    fn lines_owned(self) -> impl Iterator<Item = String>;

    /// 带所有权地拆分字符串一次
    /// * 🎯无空间开销拆分字符串为两半
    /// * 🚩默认拆分从左往右（索引从小到大）第一个图式
    ///   * 📌若未找到图式，则返还自身
    /// * ⚡可避免拷贝字符串
    fn split_owned_once<Pattern: PatternWithLen>(self, pat: Pattern) -> Result<(Self, Self), Self>;

    /// 带所有权地按行拆分字符串一次
    /// * 🎯无空间开销拆分字符串为两行
    /// * 📄参考[`SplitOwned::split_char_once_owned`]
    fn split_ln_owned_once(self) -> Result<(Self, Self), Self>;
}

impl SplitOwned for String {
    fn split_owned<Pattern: PatternWithLen>(
        self,
        pattern: Pattern,
    ) -> impl Iterator<Item = String> {
        IterSplitCharOwned {
            residual: Some(self),
            pattern,
        }
    }

    fn split_owned_once<Pattern: PatternWithLen>(
        mut self,
        pattern: Pattern,
    ) -> Result<(Self, Self), Self> {
        match pattern.find_with_len(&self) {
            Some((index_begin_of_delim, len_delim)) => {
                let index_end_of_delim = index_begin_of_delim + len_delim;
                debug_assert!(
                    self.is_char_boundary(index_end_of_delim),
                    "不会发生：find_delim在{self:?}中找到的索引{index_begin_of_delim}应该在合法UTF-8位置"
                );
                // 拆分出剩余字符串
                let right = self.split_off(index_end_of_delim);
                // 截断，抛掉自身所在分隔符
                self.truncate(index_begin_of_delim);
                // 返回
                Ok((self, right))
            }
            // 没分隔符了⇒返回「自身@错误」
            None => Err(self),
        }
    }

    fn lines_owned(self) -> impl Iterator<Item = String> {
        LinesOwned {
            residual: Some(self),
        }
    }

    fn split_ln_owned_once(mut self) -> Result<(Self, Self), Self> {
        match LinesOwned::find_delim(&self) {
            Some((index_begin_of_delim, len_delim)) => {
                let index_end_of_delim = index_begin_of_delim + len_delim;
                debug_assert!(
                    self.is_char_boundary(index_end_of_delim),
                    "不会发生：find_delim在{self:?}中找到的索引{index_begin_of_delim}应该在合法UTF-8位置"
                );
                // 拆分出剩余字符串
                let right = self.split_off(index_begin_of_delim + len_delim);
                // 抛掉分隔符
                self.truncate(index_begin_of_delim);
                Ok((self, right))
            }
            // 没分隔符了⇒返回自身所持有的字符串
            None => Err(self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{f_tensor, macro_once};

    #[test]
    fn split_char_owned() {
        fn test(c: char, s: impl ToString) {
            let s = s.to_string();
            // 一次拆分
            let cloned_split = s
                .split_once(c)
                .map(|(a, b)| (a.to_owned(), b.to_owned()))
                .ok_or_else(|| s.to_owned());
            let owned_split = s.clone().split_owned_once(c);
            assert_eq!(
                cloned_split, owned_split,
                "两种方式拆分不等：\ncloned = {cloned_split:?}\n!=\nowned = {owned_split:?}\nc = {c:?}"
            );
            // 多次拆分
            let cloned_split = s.split(c).map(ToString::to_string).collect::<Vec<_>>();
            let owned_split = s.clone().split_owned(c).take(0xff).collect::<Vec<_>>();
            assert_eq!(
                cloned_split, owned_split,
                "两种方式拆分不等：\ncloned = {cloned_split:?}\n!=\nowned = {owned_split:?}\nc = {c:?}"
            );
        }
        f_tensor! {
            test;
            '\r' '\n' '\t';
            "中文123🤣👉⇑🤡↑\nEnglish😆\nあ💭this\nYou!\r\n\t \x121\n"
            "r \r n \n rn \r\n换行最后有内容"
            "换行最后无内容\r"
            "换行最后无内容\n"
            "换行最后无内容\r\n"
        };
    }

    #[test]
    fn lines_owned() {
        fn test(s: impl ToString) {
            let s = s.to_string();
            // 拆分一次
            let cloned_split = 'cloned: {
                // naive实现：拆分`\n`或`\r\n`
                let Some(i_lf) = s.find('\n') else {
                    break 'cloned Err(s.to_owned());
                };
                const LEN_LF: usize = "\n".len();
                let left_i = match s.find("\r\n") {
                    Some(i_crlf) if i_crlf == i_lf - LEN_LF => i_crlf,
                    _ => i_lf,
                };
                Ok((s[..left_i].to_owned(), s[i_lf + 1..].to_owned()))
            };
            let owned_split = s.clone().split_ln_owned_once();
            assert_eq!(
                cloned_split, owned_split,
                "两种方式拆分不等：\ns = {s:?}\ncloned = {cloned_split:?}\n!=\nowned = {owned_split:?}"
            );
            // 拆分多次
            let cloned_lines = s.lines().map(ToString::to_string).collect::<Vec<_>>();
            let owned_lines = s.clone().lines_owned().take(0xffff).collect::<Vec<_>>();
            assert_eq!(
                cloned_lines, owned_lines,
                "两种方式拆分不等：\ns = {s:?}\ncloned = {cloned_lines:?}\n!=\nowned = {owned_lines:?}"
            );
        }
        macro_once! {
            macro test( $($input:expr)* ) {
                $(test($input);)*
            }
            "中文123🤣👉⇑🤡↑\nEnglish😆\nあ💭this\nYou!\r\n\t \x121\n"
            "r \r n \n rn \r\n换行最后有内容"
            "俩\\n \n\n 后边"
            "俩\\r \r\r 后边"
            "\\r\\n \r\n 后边"
            "仨\\n \n\n\n 后边"
            "仨\\r \r\r\r 后边"
            "\\r\\n\\r \r\n\r 后边"
            "后边没有：俩\\n \n\n"
            "后边没有：俩\\r \r\r"
            "后边没有：\\r\\n \r\n"
            "后边没有：仨\\n \n\n\n"
            "后边没有：仨\\r \r\r\r"
            "后边没有：\\r\\n\\r \r\n\r"
            "换行最后无内容\r"
            "换行最后无内容\n"
            "换行最后无内容\r\n"
            "\r".repeat(0xff)
            "\n".repeat(0xff)
            "\r\n".repeat(0xff)
            " \r".repeat(0xff)
            " \n".repeat(0xff)
            " \r\n".repeat(0xff)
            " \r ".repeat(0xff)
            " \n ".repeat(0xff)
            " \r\n ".repeat(0xff)
            "\r ".repeat(0xff)
            "\n ".repeat(0xff)
            "\r\n ".repeat(0xff)
        }
    }
}
