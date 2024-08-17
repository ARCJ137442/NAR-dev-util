//! 与各种字符串处理相关

// 导入并公开导出所有 //
crate::pub_mod_and_pub_use! {
    // as_str_ref
    as_str_ref
    // 字符数组切片
    char_slices
    // to_debug
    to_debug
}

// 前后缀匹配
// ! 【2024-03-18 22:00:28】单独列举：其中导出了宏
pub mod x_fix_match;
pub use x_fix_match::*;

// join
pub mod join;
pub use join::*;

// 带所有权拆分
pub mod split_owned;
pub use split_owned::*;
