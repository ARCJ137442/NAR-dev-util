//! 与各种字符串处理相关

// join
mod join;
pub use join::*;

// 前缀匹配
// * ✨现在内置了「线性查找」的解决方案，模块层面上暂时不需要[`algorithms`]特性了
mod prefix_match;
pub use prefix_match::*;

// 字符数组切片
mod char_slices;
pub use char_slices::*;
