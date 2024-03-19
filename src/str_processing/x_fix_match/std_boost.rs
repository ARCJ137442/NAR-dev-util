//! 用以增强标准库的一些方法
//! * 🎯最初由「`&[char]`要支持`&str`前后缀匹配」而来

use crate::if_return;

/// 用于为「字符数组切片」添加对「静态字串」的前缀匹配功能
pub trait StartsWithStr {
    /// 检查自身是否以指定静态字串（`&str`）开头
    /// * 📌类似[`[T]::starts_with`]方法，但会**逐个字符比对字符串**
    fn starts_with_str(&self, needle: &str) -> bool;
}

impl StartsWithStr for [char] {
    fn starts_with_str(&self, needle: &str) -> bool {
        // 空字串总是为true
        if_return! { needle.is_empty() => true }
        // 空自身总是为false
        if_return! { self.is_empty() => false }
        // 生成字符迭代器
        let mut needle_chars = needle.chars();
        // 逐个检查自身字符（不从字符串处检查，避免不必要的越界检查）
        for c in self.iter() {
            // 从 needle 中取下一个字符
            match needle_chars.next() {
                // 有且字符相等⇒继续
                Some(c2) if *c == c2 => (),
                // 没有字符⇒true | 比自身短
                None => return true,
                // 否则⇒返回 false
                _ => return false,
            }
        }
        // 检查完成⇒返回 true
        true
    }
}

/// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    use crate::asserts;

    /// 测试 &[char]是否支持&str的前缀匹配
    #[test]
    fn test_starts_with_str() {
        macro_rules! chars {
            ($( $char:literal )*) => {
                [$( $char ),*]
            };
        }
        asserts! {
            chars!['a' 'b' 'c'].starts_with_str("abc")
            chars!['a' 'b' 'c'].starts_with_str("ab")
            chars!['a' 'b' 'c'].starts_with_str("a")
            chars!['a' 'b' 'c'].starts_with_str("")
        }
    }
}
