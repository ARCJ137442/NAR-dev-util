//! 与「浮点处理」有关的实用工具

/// 「0-1」实数
/// 📌通过特征为浮点数添加「0-1 限制」方法
///   * 📝而非直接`impl FloatPrecision`：孤儿规则
pub trait ZeroOneFloat {
    /// 判断是否在范围内
    fn is_in_01(&self) -> bool;
    /// 验证「0-1」合法性
    /// * 📌短暂借走所有权，比对后归还
    /// * ⚠️若不在范围内，则产生panic
    fn validate_01(self) -> Self;
}

/// 批量实现「0-1」实数
/// * 🚩【2024-03-18 21:32:40】现在使用**宏**而非依赖「默认精度」超参数
/// * 📌减少重复代码，即便实际上只需实现两个类型
macro_rules! impl_zero_one_float {
    ($($t:tt)*) => {$(
        /// 实现
        impl ZeroOneFloat for $t {
            fn is_in_01(&self) -> bool {
                *self >= 0.0 && *self <= 1.0
            }
            fn validate_01(self) -> Self {
                // * 📝Clippy：可以使用「区间包含」而非「条件组合」
                if !(0.0..=1.0).contains(&self) {
                    panic!("「0-1」区间外的值（建议：`0<x<1`）");
                }
                self
            }
        }
    )*};
}
impl_zero_one_float! {
    f32
    f64
}

/// 单元测试/「0-1」实数
#[cfg(test)]
mod tests_01_float {
    use crate::{fail_tests, show};

    use super::*;
    /// 辅助用测试宏/成功测试
    /// ! 📝`, $(,)?`这里的「,」代表的不是「分隔表达式」，而是「模式中的`,`」
    /// * 故应去除这前边的「,」
    macro_rules! test_valid {
        // 📝实际上这里边啥括号都可以
        [$($num:expr),* $(,)?] => {
            $(
                let v = $num.validate_01();
                assert_eq!(v, $num);
                show!($num.validate_01());
            )*
        };
    }

    #[test]
    fn test_01_float_valid() {
        test_valid![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
    }

    /// 辅助用测试宏/失败测试
    /// ! 📝`, $(,)?`这里的「,」代表的不是「分隔表达式」，而是「模式中的`,`」
    /// * 故应去除这前边的「,」
    macro_rules! test_all_invalid {
        ($($name:ident => $num:expr),* $(,)?) => {
            // 直接用`fail_tests!`生成失败测试
            fail_tests!{
                $(
                    $name ($num.validate_01());
                )*
            }
        };
    }

    test_all_invalid! {
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
