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
    /// 获取第一个匹配字符的索引位置和长度
    fn find_with_len(&self, haystack: &str) -> Option<(usize, usize)>;

    /// 是否忽略最后一个空子串
    /// * 🎯同时适配「拆分行」与「拆分普通图式」
    ///   * 📄「拆分行」在`"abc\n"`仅拆分出`["abc"]`而不会拆出`""`
    /// * 📜默认为否：禁用此规则
    const IGNORE_FINAL_EMPTY: bool = false;
}

/// 统一对闭包实现
/// * 📌【2024-08-17 21:57:27】此处基于可copy的[`Fn`]
impl<F: Fn(char) -> bool> PatternWithLen for F {
    fn find_with_len(&self, haystack: &str) -> Option<(usize, usize)> {
        match haystack.find(self) {
            // * 🚩长度：获取字符串在索引i之后第一个字符的长度
            Some(i) => Some((i, next_char_len(haystack, i)?)),
            None => None,
        }
    }
}

fn next_char_len(haystack: &str, i: usize) -> Option<usize> {
    Some(haystack[i..].chars().next()?.len_utf8())
}

impl PatternWithLen for char {
    fn find_with_len(&self, haystack: &str) -> Option<(usize, usize)> {
        haystack.find(*self).map(|i| (i, self.len_utf8()))
    }
}

impl PatternWithLen for &str {
    fn find_with_len(&self, haystack: &str) -> Option<(usize, usize)> {
        // ! ❌【2024-08-17 22:57:19】禁用空字串的使用
        // * 🔗参考：<https://github.com/rust-lang/rust/issues/33882>
        assert!(!self.is_empty(), "Empty pattern is not allowed. Discussions see <https://github.com/rust-lang/rust/issues/33882>");
        haystack.find(self).map(|i| (i, self.len()))
    }
}

/// 用于作为「换行」的搜索图式
#[derive(Debug, Clone, Copy)]
pub struct NewLine;

impl PatternWithLen for NewLine {
    const IGNORE_FINAL_EMPTY: bool = true;
    /// 参照[`core::str::Lines`]内部的`LinesMap`（私有）制作
    /// * 📌返回`(起始索引, 子串长度)`
    /// * 🚩先拿到换行`\n`，然后试着到回头拿回车`\r`
    fn find_with_len(&self, haystack: &str) -> Option<(usize, usize)> {
        const LEN_CR: usize = "\r".len();
        const LEN_LF: usize = "\n".len();
        const LEN_CRLF: usize = "\r\n".len();
        // 先拿到换行符索引
        let lf_index = haystack.find('\n')?;
        if lf_index >= LEN_CR {
            // 若有可能，尝试拿回车符
            let cr_index = lf_index - LEN_CR;
            // ⚠️此处单凭相减得到的索引，可能不是合法UTF-8位置
            if haystack.is_char_boundary(cr_index) && haystack[cr_index..lf_index] == *"\r" {
                // 换行回车
                return Some((cr_index, LEN_CRLF));
            }
        }
        // 不然只有换行
        Some((lf_index, LEN_LF))
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
            // 空字串情况⇒单独处理
            // * 🚩降级为「遍历所有字符」
            // ! ❌仍然无法与`str::split`匹配
            //   * 🚩【2024-08-17 22:54:59】目前选择「注释掉逻辑&panic」禁止此情形
            //   * 🔗参考：<https://github.com/rust-lang/rust/issues/33882>
            // Some((index_begin_of_delim, 0)) => {
            //     let index_next_char = next_char_len(residual, index_begin_of_delim)?;
            //     residual.split_off(index_next_char)
            // }
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
        if Pattern::IGNORE_FINAL_EMPTY && residual.is_empty() {
            // 剩余的字符串为空，则直接返回
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
    ///
    /// # Panics
    ///
    /// ❌【2024-08-17 22:59:51】目前**禁止输入空字符串** `""`：效果与对应[`str::split`]不一致，且使用场合少
    /// * ⚠️Empty `&str` as pattern is forbidden. Otherwise, the program will panic.
    /// * 🚩建议：在传入该方法前预先判空
    /// * 🚧后续若有使用需求，才考虑加入
    /// * 📌主要堵点：[`str::split`]不一致的「前后空白子串」
    ///   * 🔗参考：<https://github.com/rust-lang/rust/issues/33882>
    ///
    /// ## Example
    ///
    /// cloned = `["", "中", "文", "1", "2", "3", "🤣", "👉", "⇑", "🤡", "↑", "\n", "E", "n", "g", "l", "i", "s", "h", "😆", "\n", "あ", "💭", "t", "h", "i", "s", "\n", "Y", "o", "u", "!", "\r", "\n", "\t", " ", "\u{12}", "1", "\n", ""]`
    /// !=
    /// owned = `["中", "文", "1", "2", "3", "🤣", "👉", "⇑", "🤡", "↑", "\n", "E", "n", "g", "l", "i", "s", "h", "😆", "\n", "あ", "💭", "t", "h", "i", "s", "\n", "Y", "o", "u", "!", "\r", "\n", "\t", " ", "\u{12}", "1", "\n"]`
    fn split_owned<Pattern: PatternWithLen>(self, pat: Pattern) -> impl Iterator<Item = String>;

    /// 带所有权地拆分字符串的行
    /// * 🎯无空间开销地拆分字符串
    ///   * 📄场景：一个数十Kb级大小的JSON文本要拆成两行，需要尽可能避免内容复制
    /// * ⚡可避免拷贝字符串
    fn lines_owned(self) -> impl Iterator<Item = String> {
        self.split_owned(NewLine)
    }

    /// 带所有权地拆分字符串一次
    /// * 🎯无空间开销拆分字符串为两半
    /// * 🚩默认拆分从左往右（索引从小到大）第一个图式
    ///   * 📌若未找到图式，则返还自身
    /// * ⚡可避免拷贝字符串
    ///
    /// # Panics
    ///
    /// ❌【2024-08-17 22:59:51】目前禁止输入**空字符串**，因效果与对应[`str::split`]不一致
    /// * ⚠️Empty `&str` as pattern is forbidden. Otherwise, the program will panic.
    /// * 🚧后续若有使用需求，才考虑加入
    /// * 📌主要堵点：[`str::split`]不一致的「前后空白子串」
    ///   * 🔗参考：<https://github.com/rust-lang/rust/issues/33882>
    fn split_owned_once<Pattern: PatternWithLen>(self, pat: Pattern) -> Result<(Self, Self), Self>;

    /// 带所有权地按行拆分字符串一次
    /// * 🎯无空间开销拆分字符串为两行
    /// * 📄参考[`SplitOwned::split_char_once_owned`]
    fn split_line_owned_once(self) -> Result<(Self, Self), Self> {
        self.split_owned_once(NewLine)
    }
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{f_tensor, macro_once};

    #[test]
    fn split_owned_char() {
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
    fn split_owned_ref_str() {
        fn test(c: &str, s: impl ToString) {
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
            "\r" "\n" "\r\n" "\t" /* "" */ "🤣" "n";
            "中文123🤣👉⇑🤡↑\nEnglish😆\nあ💭this\nYou!\r\n\t \x121\n"
            "r \r n \n rn \r\n换行最后有内容"
            "换行最后无内容\r"
            "换行最后无内容\n"
            "换行最后无内容\r\n"
        };
    }

    /// 禁止对空字符串展开迭代
    #[test]
    #[should_panic]
    fn empty_str_pattern_is_forbidden() {
        for _ in "abc".to_string().split_owned("") {}
    }

    #[test]
    fn split_owned_fn() {
        fn test(pat: impl Fn(char) -> bool, s: impl ToString) {
            let s = s.to_string();
            // 一次拆分
            let cloned_split = s
                .split_once(&pat)
                .map(|(a, b)| (a.to_owned(), b.to_owned()))
                .ok_or_else(|| s.to_owned());
            let owned_split = s.clone().split_owned_once(&pat);
            assert_eq!(
                cloned_split, owned_split,
                "两种方式拆分不等：\ncloned = {cloned_split:?}\n!=\nowned = {owned_split:?}"
            );
            // 多次拆分
            let cloned_split = s.split(&pat).map(ToString::to_string).collect::<Vec<_>>();
            let owned_split = s.clone().split_owned(&pat).take(0xff).collect::<Vec<_>>();
            assert_eq!(
                cloned_split, owned_split,
                "两种方式拆分不等：\ncloned = {cloned_split:?}\n!=\nowned = {owned_split:?}"
            );
        }
        f_tensor! {
            test;
            char::is_whitespace
            char::is_alphabetic
            char::is_alphanumeric,
            { |c:char| c.is_ascii() }
            ;
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
            let owned_split = s.clone().split_line_owned_once();
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
