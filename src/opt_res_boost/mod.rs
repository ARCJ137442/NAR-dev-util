//! 用于增强Rust的[`Option`][`Result`]类型
//! * 🎯尤其对「从其它地方接收到一个不同类型的Result，需要转换成另一种Result并返回」的场景有用
//! * 📄`Result<T, E1>` --> `Result<T, E2>` --> `?`
//! * 🚩现在通用化为「opt(ion)_res(ult)_boost」，以备后续扩展功能
//!   * ❌最初尝试用于「unwrap时能提供错误信息」，简化`match r {..., Err(e) => panic!("{e}")}`的情形
//!     * 📝Rust自身就对[`Result::unwrap`]有提示："called `Result::unwrap()` on an `Err` value: ..."

// 增强Option
mod option;
pub use option::*;

// 增强Result
mod result;
pub use result::*;
