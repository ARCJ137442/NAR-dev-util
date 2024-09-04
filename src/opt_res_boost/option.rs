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
        // 仅在self为None、other不为None时，将f_value_gen(other)的值赋给self
        if let (None, Some(v)) = (&self, other) {
            *self = Some(f_value_gen(v))
        }
    }
}
