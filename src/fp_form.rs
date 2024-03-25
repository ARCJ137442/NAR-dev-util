//! FP形式
//! * 🎯提供一些非常「函数式」的语法支持
//!
//! ## ✨链式语法
//!
//! * 🎯对任何值使用链式语法
//! * 🚩使用闭包（最终会被内联）实现
//! * 📝评价：异常函数式的写法

/// 用于完全铺开实现「链式调用」的特征
/// * 🎯将`f1(f2(x), y)`重整成`x.f(f2).f(|v| f1(v, y))`
pub trait FpForm {
    /// 链式调用语法
    /// * 🚩传入闭包以实现「链式调用」
    /// * 🎯支持「无限链式调用」语法
    /// * 📄形如：`self.f(|v| XXX(v)).f(|xxx_v| YYY(v))`
    #[inline(always)]
    fn f(self, function: impl FnOnce(Self) -> Self) -> Self
    where
        // 仅针对「有大小」类型
        // * 📌只会对`dyn T`等少数类型无效
        Self: Sized,
    {
        function(self)
    }

    /// 链式调用语法，但名称不占用关键字
    /// * 🎯让代码更显美观
    #[doc(alias = "f")]
    #[inline(always)]
    fn to(self, function: impl FnOnce(Self) -> Self) -> Self
    where
        Self: Sized,
    {
        self.f(function)
    }

    /// 链式调用语法，但名称不占用关键字
    /// * 🎯让代码更显美观
    #[doc(alias = "f")]
    #[inline(always)]
    fn deal(self, f: impl FnOnce(Self) -> Self) -> Self
    where
        Self: Sized,
    {
        self.f(f)
    }
}

/// 直接对所有类型实现
impl<T> FpForm for T {}

/// 单元测试
#[cfg(test)]
mod test {
    use super::*;
    use crate::{asserts, pipe};
    use std::ops::Add;

    #[test]
    fn test() {
        // 简单示例
        let a = -1; // 1
        let abs_a = a.f(i32::abs); // 1
        let proceed = abs_a.f(|a| a + 1); // 2

        // 定义一些可执行对象，其名称反映适用场景
        fn a_very_complex_and_long_and_tedious_add_one<N: Add<i32, Output = i32>>(x: N) -> i32 {
            x + 1
        }
        fn a_very_complex_and_long_and_tedious_add_function<N: Add<N, Output = N>>(
            x: N,
            y: N,
        ) -> N {
            x + y
        }
        let a_very_complex_and_long_and_tedious_add_a_closure = |x: i32| x + a;

        // 展示调用过程
        let p_proceed = proceed // 2
            // 加上自身 => 4
            .f(|x| a_very_complex_and_long_and_tedious_add_function(x, x))
            // +1 => 5
            .f(a_very_complex_and_long_and_tedious_add_one) // 5
            // +a => 6
            .f(a_very_complex_and_long_and_tedious_add_a_closure); // 4

        // 基于`pipe`复现一下
        let pipe_proceed = pipe! {
            proceed
            // * ⚠️注意：此处多元插值，重复拷贝了俩表达式
            => a_very_complex_and_long_and_tedious_add_function(_, _)
            => a_very_complex_and_long_and_tedious_add_one
            => a_very_complex_and_long_and_tedious_add_a_closure
        };

        // 检验结果
        asserts! {
            a => -1,
            abs_a => 1,
            proceed => 2,
            p_proceed => 4,
            p_proceed => pipe_proceed,
        }
    }
}
