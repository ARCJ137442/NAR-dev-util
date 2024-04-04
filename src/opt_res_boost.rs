//! 用于增强Rust的[`Option`][`Result`]类型
//! * 🎯尤其对「从其它地方接收到一个不同类型的Result，需要转换成另一种Result并返回」的场景有用
//! * 📄`Result<T, E1>` --> `Result<T, E2>` --> `?`
//! * 🚩现在通用化为「opt(ion)_res(ult)_boost」，以备后续扩展功能
//!   * ❌最初尝试用于「unwrap时能提供错误信息」，简化`match r {..., Err(e) => panic!("{e}")}`的情形
//!     * 📝Rust自身就对[`Result::unwrap`]有提示："called `Result::unwrap()` on an `Err` value: ..."

use std::{convert::identity, fmt::Debug};

/// 实用封装：`Result<T, String>`
/// * 🎯用于表示一些「没必要单独立一个结构来存储」的「轻量级[`Result`]」
///   * 📌这些类型一般只需一个字符串[`String`]存储错误消息
///   * 📌或者简单统一多个来源的不同类型错误
pub type ResultS<T> = Result<T, String>;

/// 用于为一般的[`Option`]添加功能
pub trait OptionBoost<T>: Sized {
    /// 🚩在自身为`None`时执行代码，并返回自身
    /// * 🎯填补[`Option`]「只有对[`Some`]的`inspect`而没有对[`None`]的`inspect`」的情况
    fn inspect_none(self, none_handler: impl FnOnce()) -> Self;

    /// 强制将自身转换为[`None`]
    /// * 📌销毁内部的值
    fn none(self) -> Self;

    /// 在自身为[`Some`]时，执行函数处理其内值，否则返回指定的值
    /// * 📌实际上为`self.map(f).unwrap_or(else_value)`的简写
    fn map_unwrap_or<U>(self, f: impl FnOnce(T) -> U, default: U) -> U;

    // ! 📝有关`&Option<T>` -> `Option<&T>`的「引用内置」转换，可使用[`Option::as_ref`]

    /// 实现从其它[`Option`]的「空值合并」操作
    /// * ✨只需使用「并入值」的不可变引用，后续要合并时调用「值生成函数」
    /// * ⚡最大程度惰性生成值（如「惰性拷贝」）
    fn coalesce<F>(&mut self, other: &Self, f: F)
    where
        F: FnOnce(&T) -> T;

    /// 实现从其它[`Option`]的「空置拷贝合并」操作
    /// * ✨只需使用「并入值」的不可变引用，后续要合并时才拷贝已有值
    /// * ⚡最大程度惰性拷贝值
    fn coalesce_clone(&mut self, other: &Self)
    where
        T: Clone,
    {
        self.coalesce(other, T::clone)
    }
}

impl<T> OptionBoost<T> for Option<T> {
    fn inspect_none(self, none_handler: impl FnOnce()) -> Self {
        if self.is_none() {
            none_handler()
        }
        self
    }

    fn none(self) -> Self
    where
        Self: Sized,
    {
        None
    }

    #[inline]
    #[must_use]
    fn map_unwrap_or<U>(self, f: impl FnOnce(T) -> U, default: U) -> U {
        // self.map(f).unwrap_or(else_value)
        match self {
            Some(t) => f(t),
            None => default,
        }
    }

    #[inline]
    fn coalesce<F>(&mut self, other: &Self, f_value_gen: F)
    where
        F: FnOnce(&T) -> T,
    {
        // 仅在self为None、other不为None时，将other的值赋给self
        if let (None, Some(v)) = (&self, other) {
            *self = Some(f_value_gen(v))
        }
    }
}

/// 用于为一般的[`Result`]添加功能
/// * 🎯用于`Result<T, E>`
pub trait ResultBoost<T, E> {
    /// 使用两个「转换器」函数，将[`Result`]的[`Ok`]和[`Err`]分别做映射
    /// * 🎯用于简化`Ok(..) => Ok(..), Err(..) => Err(..)`的情形
    /// * 📝【2024-03-20 21:50:44】此处使用[`FnMut`]以便允许在闭包中修改包外变量
    fn transform<T2, Error2>(
        self,
        transformer_ok: impl FnMut(T) -> T2,
        transformer_err: impl FnMut(E) -> Error2,
    ) -> Result<T2, Error2>;

    /// 使用一个「转换器」函数，将内容相同的[`Result`]的错误转换成另一种错误
    /// * 🎯用于「从其它地方调用方法返回不同类型的错误，但调用处希望仍然能使用`?`上抛」的情况
    /// * 📌亦可使用[`transform`] + [`core::convert::identity`]
    ///   * ✅【2024-03-24 00:22:54】现在提供默认实现：直接限制`Self: Sized`
    ///   * 📝基本所有[`Result`]类型都是[`Sized`]的，除非`dyn Trait`之类
    #[inline(always)]
    fn transform_err<Error2>(self, transformer: impl FnMut(E) -> Error2) -> Result<T, Error2>
    where
        Self: Sized,
    {
        self.transform(identity, transformer)
    }

    /// 将错误自动转换为字符串，并返回一个字符串形式[`Err`]的[`Result`]
    /// * 🎯用于快速转换成`Result<T, String>`
    /// * 🎯常用于一些轻量级[`Result`]使用场景
    ///   * 📌需要使用`?`上报错误，并且需要尽可能详细的错误信息
    ///   * 📌不希望引入大量的`e.to_string`，但`错误类型::to_string`函数指针又用不了
    #[inline(always)]
    fn transform_err_debug(self) -> Result<T, String>
    where
        Self: Sized,
        E: Debug,
    {
        self.transform_err(|e| format!("{e:?}"))
    }

    /// 将错误自动转换为字符串
    /// * 📌但相比[`ResultBoost::transform_err_debug`]用到了[`ToString`]特征
    ///   * ✅对[`Display`]也可用：前者自动实现了[`ToString`]
    #[inline(always)]
    fn transform_err_string(self) -> Result<T, String>
    where
        Self: Sized,
        E: ToString,
    {
        self.transform_err(|e| e.to_string())
    }

    /// 调转[`Ok`]与[`Err`]的类型
    /// * 🎯从`Result<T, E>`调转成`Result<E, T>`
    /// * 📌内部值不变
    fn flip(self) -> Result<E, T>;

    /// 在自身为[`Ok`]时返回带有内部值的[`Some`]，否则执行某个函数并返回[`None`]
    /// * 🎯用于「返回内容/报告错误」
    fn ok_or_run(self, f: impl FnOnce(E)) -> Option<T>;
}

/// 用于为「奇异[`Result`]」（`Ok`、`Err`类型相同）添加功能
/// * 🎯用于`Result<TorE, TorE>`
/// * 📌只有唯一的泛型参数`TorE`
pub trait ResultBoostSingular<TorE> {
    /// 抛去类型，无论是[`Ok`]还是[`Err`]，均解包其中的值
    fn collapse(self) -> TorE;
}

impl<T, E> ResultBoost<T, E> for Result<T, E> {
    #[inline]
    fn transform<T2, Error2>(
        self,
        mut transformer_ok: impl FnMut(T) -> T2,
        mut transformer_err: impl FnMut(E) -> Error2,
    ) -> Result<T2, Error2> {
        match self {
            Ok(ok) => Ok(transformer_ok(ok)),
            Err(err) => Err(transformer_err(err)),
        }
    }

    #[inline]
    fn flip(self) -> Result<E, T> {
        match self {
            Ok(v) => Err(v),
            Err(v) => Ok(v),
        }
    }

    #[inline]
    #[must_use]
    fn ok_or_run(self, f: impl FnOnce(E)) -> Option<T> {
        match self {
            Ok(v) => Some(v),
            Err(e) => {
                f(e);
                None
            }
        }
    }
}

impl<T> ResultBoostSingular<T> for Result<T, T> {
    #[inline]
    fn collapse(self) -> T {
        match self {
            Ok(v) | Err(v) => v,
        }
    }
}

/// 单元测试
#[cfg(test)]
mod test {
    use super::*;
    use crate::asserts;

    /// 测试[`Result::transform_err`]
    #[test]
    fn transform_err() {
        // 基础功能
        asserts! {
            // [`Ok`]不会发生转换
            Result::<i32, &str>::Ok(1)
                .transform_err(|_| 1) => Ok(1)

            // [`Err`]才会发生转换
            Result::<i32, &str>::Err("这是个错误")
                .transform_err(|err| err.chars().count()) => Err(5)
            Result::<i32, &str>::Err("这是个错误") // ↓自动转换为字符串
                .transform_err_debug() => Err(format!("{:?}", "这是个错误"))
            // [`Ok`]不会发生转换
            Result::<usize, usize>::Ok(0)
                .transform_err(|err| err + 1) => Ok(0)
            // [`Err`]才会发生转换
            Result::<usize, usize>::Err(0)
                .transform_err(|err| err + 1) => Err(1)
            Result::<usize, usize>::Err(0) // ↓自动转换为字符串
                .transform_err_string() => Err("0".into())
        }

        // 场景测试
        type MyResult = Result<bool, String>;
        fn is_even_of_text(text: &str) -> MyResult {
            // 一行解析并尝试上抛错误
            let parsed = text.parse::<i32>().transform_err(|err| format!("{err}"))?;
            // 直接开始业务代码
            Ok(parsed & 1 == 0)
        }

        asserts! {
            // Ok用例
            is_even_of_text("1") => Ok(false)
            is_even_of_text("0") => Ok(true),
            // Err用例
            is_even_of_text("err") => Err("invalid digit found in string".into()),
            is_even_of_text("这一定会发生错误！") => @ Err(..),
        }
    }

    #[test]
    fn flip() {
        // 基础功能
        asserts! {
            // `Ok` => `Err`
            Result::<usize, usize>::Ok(1)
                .flip() => Err(1),
            // `Err` => `Ok`
            Result::<usize, &str>::Err("value")
                .flip() => Ok("value"),
        }
    }

    #[test]
    fn collapse() {
        // 基础功能
        asserts! {
            Result::<usize, usize>::Ok(1)
                .collapse() => 1,
            Result::<&str, &str>::Ok("str")
                .collapse() => "str",
        }
    }
}
