//! 用于快速转换[`Result`]类型
//! * 🎯尤其对「从其它地方接收到一个不同类型的Result，需要转换成另一种Result并返回」的场景有用
//! * 📄`Result<T, E1>` --> `Result<T, E2>` --> `?`

/// 用于为一般的[`Result`]添加功能
/// * 🎯用于`Result<T, E>`
pub trait ResultTransform<T, E> {
    /// 使用一个「转换器」函数，将内容相同的[`Result`]的错误转换成另一种错误
    /// * 🎯用于「从其它地方调用方法返回不同类型的错误，但调用处希望仍然能使用`?`上抛」的情况
    fn transform_err<Error2>(self, transformer: impl Fn(E) -> Error2) -> Result<T, Error2>;

    /// 调转[`Ok`]与[`Err`]的类型
    /// * 🎯从`Result<T, E>`调转成`Result<E, T>`
    /// * 📌内部值不变
    fn flip(self) -> Result<E, T>;
}

/// 用于为「奇异[`Result`]」（`Ok`、`Err`类型相同）添加功能
/// * 🎯用于`Result<TorE, TorE>`
/// * 📌只有唯一的泛型参数`TorE`
pub trait ResultTransformSingular<TorE> {
    /// 抛去类型，无论是[`Ok`]还是[`Err`]，均解包其中的值
    fn collapse(self) -> TorE;
}

impl<T, E> ResultTransform<T, E> for Result<T, E> {
    #[inline]
    fn transform_err<Error2>(self, transformer: impl Fn(E) -> Error2) -> Result<T, Error2> {
        match self {
            Err(old_error) => Err(transformer(old_error)),
            Ok(v) => Ok(v),
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

impl<T> ResultTransformSingular<T> for Result<T, T> {
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
    use crate::{asserts, ResultTransform, ResultTransformSingular};

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
