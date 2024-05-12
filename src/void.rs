//! # `void`功能
//! * 🎯以JS的`void XXX`语法，实现「取消函数输出」功能

/// 作为特征的[`Void`]
/// * 🚩只含有一个[`void`](`Void::void`)方法，直接通过`self.void()`语法调用
/// * 🎯后缀对标JavaScript`void XXX`语法
///
/// ## 用例
///
/// ```rust
/// use nar_dev_utils::Void;
/// fn one() -> i32 {
///     1
/// }
///
/// match 1 {
///     1 => one().void(),
///     _ => (),
/// }
/// ```
pub trait Void: Sized {
    #[inline(always)]
    fn void(self) {}
}
impl<T: Sized> Void for T {}

/// 作为函数的`void`方法
/// * 🚩直接内联Rust自带的[`drop`]
/// * 📝对于作为函数的`void`，建议使用Rust自带的`drop`
#[inline(always)]
pub fn void<T>(t: T) {
    drop(t)
}

/// # `void`宏
/// * 🎯简单地在「无需使用返回值」的情况中【取消返回值】
///   * 📄在`match`语句中解决「返回值不一致」的问题
/// * ✅与JavaScript中`void f(x)`等同
/// * ✨可在其中加入任意可执行代码，而不仅仅是单个表达式
/// * 🚩通过「块表达式+末尾分号」实现
///
/// ## 用例
///
/// ```rust
/// use nar_dev_utils::void;
/// fn one() -> i32 {
///     println!("one!");
///     1
/// }
/// assert_eq!(one(), 1);
/// match 2 {
///     1 => void!(one()),
///     _ => println!("other!"),
/// }
/// match 1 {
///     0 => void!({ dbg!("0") }),
///     1 => void![println!("1"); one()],
///     _ => println!("other!"),
/// }
/// ```
#[macro_export]
macro_rules! void {
    ($($code:tt)*) => {{
        $($code)*;
    }};
}
