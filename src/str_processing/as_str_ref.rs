//! 「作为静态字串引用」
//! * 🎯用于各种【需要兼容`String`、`&str`、`&String`到`&str`】的场景

use crate::macro_once;

/// 将自身转换为 `&str`
/// * 🎯用来表达类似TypeScript`String | str | &String | &str | ...`的语义
///   * 用来统一兼容各种字符串类型
/// * 📌后续写函数的时候，只需一个`s: impl AsStrRef`
/// * ✨**再也不用考虑对`String`、`&str`的通用兼容问题了**
///
/// ! ❌`Deref<Target=str>`/`Deref<Target=&str>`无法满足上述兼容要求
/// * 📝无法同时满足`String`与`&str`：满足了一个，另一个又不满足了
#[deprecated(note = "use `AsRef<str>` instead")]
pub trait AsStrRef {
    /// 统一、通用、方便地转换为`&str`
    /// * 🎯不管自己是什么类型，反正内容都一样，就一定能转换成`&str`
    fn as_str_ref(&self) -> &str;
}
macro_once! {
    /// 批量实现上述特征的宏
    macro impl_as_str_ref($( $t: ty $(,)? )*) {
        $(
            /// ! 兼容各路类型转换到`&str`
            /// * ⚠️弃用：请使用标准库实现`AsRef<str>`
            #[allow(deprecated)]
            impl AsStrRef for $t {
                fn as_str_ref(&self) -> &str {
                    self // ! 引用自动转换全都用的`self`，可作为参数传递时却不能这样自动转换
                }
            }
        )*
    }
    String,
    &String,
    &&String,
    &&&String,
    // ! 💭【2024-03-22 21:03:50】一般到三层引用就够了
    str,
    &str,
    &&str,
    &&&str,
}
