//! 为字符串实现`into_chars`方法
//! * 📄参考：https://internals.rust-lang.org/t/is-there-a-good-reason-why-string-has-no-into-chars/19496/7
//! * 🎯最初用于「Narsese词法解析器」的「静态字串→字符迭代器」的完全转换
//!   * 类型：`&str` -> `impl Iterator<Item = char>`
pub trait IntoChars {
    /// 将自身转换为字符迭代器，获取自身所有权
    fn into_chars(self) -> impl Iterator<Item = char>;
}

/// 对静态字串实现`into_chars`方法
impl IntoChars for &str {
    fn into_chars(self) -> impl Iterator<Item = char> {
        self.to_owned().into_chars()
    }
}

/// 对动态字串实现`into_chars`方法
impl IntoChars for String {
    /// 迁移自<https://github.com/rust-lang/libs-team/issues/268>
    /// * ⚠️少量修改
    ///   * 🚩使用自己的「函数式迭代器」[`crate::FnIterator`]
    ///   * 📌使用闭包捕获自身作为变量，以避免「临时引用」问题
    /// * 🚩【2024-03-18 21:11:23】现在直接使用[`std::iter::from_fn`]，无需函数式迭代器
    fn into_chars(self) -> impl Iterator<Item = char> {
        let mut i = 0;
        // 创建函数式迭代器，捕获变量`i`与自身
        std::iter::from_fn(move || {
            if i < self.len() {
                let c = self[i..].chars().next().unwrap();
                i += c.len_utf8();
                Some(c)
            } else {
                None
            }
        })
    }
}
