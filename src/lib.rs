//! 一些实用工具、定义、函数
//! * 📌宏定义专门放在[`macros.rs`]中
//!   * 📄参考标准库与其它包（如`winnow`）

// 实用宏 // ! 默认启用
mod macros;

// 预引入 // ! 默认启用
mod prelude;
pub use prelude::*;

// 特性 => 模块 | 依靠特性导入并重新导出模块 //
// ! ⚠️【2024-03-18 21:44:47】已知问题：无法兼容「导出了宏的模块」
// ! 🔗参考：<https://github.com/rust-lang/rust/pull/52234>
feature_pub_mod_and_reexport! {
    // 浮点
    "floats" => floats

    // 字符串处理
    // "str_processing" => str_processing
    // ! ❌【2024-03-18 21:44:08】该模块有导出宏，故不启用

    // 迭代器
    "iterators" => iterators

    // Vec工具
    "vec_tools" => vec_tools

    // 字符串⇒字符迭代器 | IntoChars
    "into_chars" => into_chars
}

// 其它模块 //
#[cfg(feature = "str_processing")]
mod str_processing;
#[cfg(feature = "str_processing")]
pub use str_processing::*;
