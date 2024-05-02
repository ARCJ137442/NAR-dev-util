//! 将对象转换为「用Debug格式化的字符串」
//! * 🚩封装简单而冗杂的`format!("{self}")`代码
//! * 🚩使用一个非常简单的小特征
//!   * 允许使用`self.to_debug()`语法

/// 将对象转换为「用Debug格式化的字符串」
pub trait ToDebug {
    fn to_debug(&self) -> String;
}

impl<T: std::fmt::Debug> ToDebug for T {
    /// 将对象转换为「用Debug格式化的字符串」
    #[inline]
    fn to_debug(&self) -> String {
        format!("{self:?}")
    }
}

/// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{asserts, macro_once};

    #[test]
    #[allow(unused_allocation)] // 用于`Box`
    fn test() {
        // 简单对象测试
        asserts! {
            "1" => 1.to_debug(),
            "\"1\"" => "1".to_debug(),
            "'1'" => '1'.to_debug(),
            "()" => ().to_debug(),
        }

        // 大规模测试
        macro_once! {
            macro testset($($e:expr $(,)?)*) {
                asserts! {
                    $(
                        format!("{:?}", &$e) => $e.to_debug(),
                    )*
                }
            }
            1 2 3 4 5 6 7 8 9 10,
            (1, 2), [1, 2, 3],
            (1, 2, (1, 2)),
            "string", 'c',
            ("string".to_string()),
            vec![1, 2, 3],
            Box::new(1),
            Box::new(Box::new(0)),
            &[1, 2, 3]
        }
    }
}
