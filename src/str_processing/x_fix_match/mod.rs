//! 用于统一支持「前缀匹配」「后缀匹配」
//! * 🎯「词法Narsese」的「前后缀分割」机制
//!   * 📄括弧匹配：`(` <=> `)`
//!   * 📄前后缀匹配「陈述系词」（两可情况）
//! * 📝此中结构的使用策略
//!   * 📌单纯 前缀匹配/后缀匹配（无需括号） ⇒ [`XFixMatchDict`]
//!   * 📌前缀匹配左括弧，映射到「右括弧」 ⇒ [`PrefixMatchDict`]
//!   * 📌后缀匹配右括弧，映射到「左括弧」 ⇒ [`SuffixMatchDict`]
//!   * 📌前缀匹配左括弧⇄后缀匹配右括弧 ⇒ [`BiFixMatchDict`]
//!
//! ! ⚠️此处无法使用[`crate::mod_and_reexport`]宏
//! * 📌原因：内部导出了宏

// 抽象特征
mod traits;
pub use traits::*;

// 元组实现
mod impl_tuple; // * 直接声明实现即可

// 词缀匹配（通用）
mod x_fix_dict;
pub use x_fix_dict::*;

// 前缀匹配
// * ✨现在内置了「线性查找」的解决方案，模块层面上暂时不需要[`algorithms`]特性了
mod prefix_match;
pub use prefix_match::*;

// 后缀匹配
mod suffix_match;
pub use suffix_match::*;

// 双向匹配
mod bi_fix_dict;
pub use bi_fix_dict::*;
