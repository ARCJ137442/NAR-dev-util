//! 与「浮点处理」有关的实用工具

use crate::macro_once;

/// 「0-1」实数
/// 📌通过特征为浮点数添加「0-1 限制」方法
///   * 📝而非直接`impl FloatPrecision`：孤儿规则
pub trait ZeroOneFloat {
    /// 判断是否在范围内
    fn is_in_01(&self) -> bool;

    /// 尝试验证「0-1」合法性
    /// * 🎯不会引发panic，而是返回一个[`Result`]
    ///   * 🚩在范围内⇒`Some(&自身)`
    ///   * 🚩在范围外⇒`Err(错误信息)`
    /// * 📝不能直接使用`Result<Self, _>`：其编译时大小未知
    fn try_validate_01(&self) -> Result<&Self, &str> {
        match self.is_in_01() {
            true => Ok(self),
            false => Err("「0-1」区间外的值（建议：`0<x<1`）"),
        }
    }

    /// 验证「0-1」合法性
    /// * 📌只使用不可变借用：仅需比较，并且`Self`大小未知
    /// * ⚠️若不在范围内，则产生panic
    /// * 🚩复用`try_validate_01`的消息
    fn validate_01(&self) -> &Self {
        // * 📝这里直接使用`unwrap`即可：报错信息会写「called `Result::unwrap()` on an `Err` value: ...」
        self.try_validate_01().unwrap()
    }
}

macro_once! {
    /// 批量实现「0-1」实数
    /// * 🚩【2024-03-18 21:32:40】现在使用**宏**而非依赖「默认精度」超参数
    /// * 📌减少重复代码，即便实际上只需实现两个类型
    macro impl_zero_one_float($($t:tt)*) {$(
        /// 实现
        impl ZeroOneFloat for $t {
            fn is_in_01(&self) -> bool {
                // * 📝Clippy：可以使用「区间包含」而非「条件组合」
                // * 📝↓下边的`=`是「小于等于」「包含右边界」的意思
                (0.0..=1.0).contains(self)
            }
        }
    )*}
    // 直接实现
    f32
    f64
}

/// 单元测试/「0-1」实数
#[cfg(test)]
mod tests_01_float {
    use super::*;
    use crate::{fail_tests, macro_once, show};

    #[test]
    fn test_01_float_valid() {
        macro_once! {
            /// 辅助用测试宏/成功测试
            ///
            /// ! 📝`, $(,)?`这里的「,」代表的不是「分隔表达式」，而是「模式中的`,`」
            /// * 故应去除这前边的「,」
            // 📝使用的时候，圆括号、方括号、花括号均可
            macro test_valid($($num:expr),* $(,)?) {
                $(
                    let v = $num.validate_01();
                    assert_eq!(*v, $num);
                    show!($num.validate_01());
                )*
            }
            // 从0.1到1.0
            0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0
        }
    }

    macro_once! {
        /// 辅助用测试宏/失败测试
        ///
        /// ! 📝`, $(,)?`这里的「,」代表的不是「分隔表达式」，而是「模式中的`,`」
        /// * 故应去除这前边的「,」
        macro test_all_invalid($($name:ident => $num:expr),* $(,)?) {
            // 直接用`fail_tests!`生成失败测试
            fail_tests!{
                $(
                    $name ($num.validate_01());
                )*
            }
        }
        // 大数
        fail_1_1 => 2.0,
        fail_3_0 => 3.0,
        fail_10_0 => 10.0,
        // 负数
        fail_n_0_1 => -0.1,
        fail_n_0_2 => -0.2,
        fail_n_2_0 => -2.0,
    }
}
