//! 一些实用工具、定义、函数
//! * 📌宏定义专门放在[`macros.rs`]中
//!   * 📄参考标准库与其它包（如`winnow`）

// 实用宏 // ! 默认启用
mod macros;

// 浮点
#[cfg(feature = "floats")]
mod floats;
#[cfg(feature = "floats")]
pub use floats::*;

// 字符串处理
#[cfg(feature = "str_processing")]
mod str_processing;
#[cfg(feature = "str_processing")]
pub use str_processing::*;

// 迭代器
#[cfg(feature = "iterators")]
mod iterators;
#[cfg(feature = "iterators")]
pub use iterators::*;

// 算法
#[cfg(feature = "algorithms")]
mod algorithms;
#[cfg(feature = "algorithms")]
pub use algorithms::*;
