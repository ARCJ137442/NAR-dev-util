//! 与各种字符串处理相关

// join
mod join;
pub use join::*;

// 前缀匹配
// ! 目前因「前缀匹配字典」在`insert`处需要二分查找，只有启用`algorithms`才能使用
#[cfg(feature = "algorithms")]
mod prefix_match;
#[cfg(feature = "algorithms")]
pub use prefix_match::*;
