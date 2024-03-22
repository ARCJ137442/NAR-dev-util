//! 用于增强Rust的[`Option`][`Result`]类型
//! * 🎯尤其对「从其它地方接收到一个不同类型的Result，需要转换成另一种Result并返回」的场景有用
//! * 📄`Result<T, E1>` --> `Result<T, E2>` --> `?`
//! * 🚩现在通用化为「opt(ion)_res(ult)_boost」，以备后续扩展功能
//!   * ❌最初尝试用于「unwrap时能提供错误信息」，简化`match r {..., Err(e) => panic!("{e}")}`的情形
//!     * 📝Rust自身就对[`Result::unwrap`]有提示："called `Result::unwrap()` on an `Err` value: ..."

use std::convert::identity;

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
    ///   * ❌【2024-03-20 21:45:25】此处不提供默认实现：consider further restricting `Self`: ` where Self: std::marker::Sized`
    fn transform_err<Error2>(self, transformer: impl FnMut(E) -> Error2) -> Result<T, Error2>;

    /// 调转[`Ok`]与[`Err`]的类型
    /// * 🎯从`Result<T, E>`调转成`Result<E, T>`
    /// * 📌内部值不变
    fn flip(self) -> Result<E, T>;
}

/// 用于为「奇异[`Result`]」（`Ok`、`Err`类型相同）添加功能
/// * 🎯用于`Result<TorE, TorE>`
/// * 📌只有唯一的泛型参数`TorE`
pub trait ResultBoostSingular<TorE> {
    /// 抛去类型，无论是[`Ok`]还是[`Err`]，均解包其中的值
    fn collapse(self) -> TorE;
}

impl<T, E> ResultBoost<T, E> for Result<T, E> {
    #[inline(always)]
    fn transform_err<Error2>(self, transformer: impl FnMut(E) -> Error2) -> Result<T, Error2> {
        self.transform(identity, transformer)
    }

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
    use crate::{asserts, ResultBoost, ResultBoostSingular};

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
            // [`Ok`]不会发生转换
            Result::<usize, usize>::Ok(0)
                .transform_err(|err| err + 1) => Ok(0)
            // [`Err`]才会发生转换
            Result::<usize, usize>::Err(0)
                .transform_err(|err| err + 1) => Err(1)
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
