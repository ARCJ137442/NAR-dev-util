/// # `first!`：匹配首个判据，并返回其值
/// * 🎯用于简写「截断性判断」结构
///   * 📌可用于简写`if-else if-else`「优先分支」结构
///   * 📌可用于简写`match 0 {_ if XXX => Z1, _ if YYY => Z2, _ => ELSE}`「伪优先分支」结构
///
/// 📝Rust的「规则宏」并不能被视作为一个类似「变量」「函数」之类能导出的量
/// * ❌无法使用常规的`pub`（相当于Julia的`export`）导出
///   * 📌需要使用`#[macro_export]`导出
///     * 📝可选的[`local_inner_macros`]：导出在当前模块中定义的「内部宏」(inner macro)。
///       * 内部宏：仅在其他宏的定义体中使用的宏
/// * ❗需要在crate层级导入，而非在定义宏的模块中导入
/// * 📝使用`#[cfg(not(test))]`标注「非测试」
///   * 🎯可防止「定义之前测试宏」导致的「文档测试（doc test）失败」
///   * ❗但也会导致在别的测试中用不了
///   * 📌SOLUTION：在文档代码块中引入`use 【库名】::*;`
///     * ❗不能用`crate` | `help: consider importing this macro`
///
/// ## 用法
///
/// ### 常规用法
///
/// ```rust
/// use nar_dev_utils::first;
/// fn see(v: &str) -> &str {
///     first! {
///         v.is_empty() => "空的！",
///         v.starts_with('0') => "以零开头！",
///         v.starts_with('1') => "以一开头！",
///         v.starts_with('2') => "以二开头！",
///         v.len() > 5 => "超长字符串！",
///         v.starts_with('3') => "以三开头！",
///         _ => "这啥玩意…", // fallback
///     }
/// }
/// ```
///
/// 将被转换成
///
/// ```rust
/// fn see(v: &str) -> &str {
///     if v.is_empty() {
///        "空的！"
///     } else if v.starts_with('0') {
///         "以零开头！"
///     } else if v.starts_with('1') {
///         "以一开头！"
///     } else if v.starts_with('2') {
///         "以二开头！"
///     } else if v.len() > 5 {
///         "超长字符串！"
///     } else if v.starts_with('3') {
///         "以三开头！"
///     } else {
///         "这啥玩意…" // fallback
///     }
/// }
/// ```
///
/// ### 结合「预处理函数」实现批量应用
///
/// ```rust
/// use nar_dev_utils::first;
/// fn see(v: &str) -> String {
///     first! {
///         // 格式：`(预处理输入) => (预处理输出)`
///         (v.starts_with) => (str::to_string);
///         '0' => "以零开头！",
///         '1' => "以一开头！",
///         '2' => "以二开头！",
///         '3' => "以三开头！",
///         _ => "这啥玩意…", // fallback
///     }
/// }
/// ```
///
/// 将被转换成
///
/// ```rust
/// fn see(v: &str) -> String {
///     // 预处理输出展开：输出统一用`str::to_string`包裹
///     str::to_string(
///         // 预处理输入展开：统一插入`v.starts_with`
///         if v.starts_with('0') {
///             "以零开头！"
///         } else if v.starts_with('1') {
///             "以一开头！"
///         } else if v.starts_with('2') {
///             "以二开头！"
///         } else if v.starts_with('3') {
///             "以三开头！"
///         } else {
///             "这啥玩意…" // fallback
///         }
///     )
/// }
/// ```
///
/// ## 用例
///
/// ```rust
/// use nar_dev_utils::{first, show, asserts};
/// let v: &str = "1";
/// // 测试1 不安排「预处理函数」 | 匹配一个无意义的值，使用匹配守卫来确定「唯一进入的分支」
/// let v = first! {
///     v.is_empty() => "空的！",
///     v.starts_with('0') => "以零开头！",
///     v.starts_with('1') => "以一开头！",
///     v.starts_with('2') => "以二开头！",
///     v.len() > 5 => "超长字符串！",
///     v.starts_with('3') => "以三开头！",
///     _ => "这啥玩意…",
/// };
/// // 测试2 使用「成员方法」预处理被匹配项 | 此时 v == "以一开头！"
/// let v2 = first! {
///     (v.starts_with) => (_);
///     '0' => "以零开头！",
///     '1' => "以一开头！",
///     '2' => "以二开头！",
///     '3' => "以三开头！",
///     _ => "这啥玩意…",
/// };
/// // 测试3 使用「闭包」处理被匹配项，同时使用「路径」处理匹配值 | 此时 v2 == "这啥玩意…"
/// let clj = |c| v2.contains(c);
/// let v3 = first! {
///     (clj) => (str::to_string);
///     '这' => "「这」在里头！",
///     '啥' => "「啥」在里头！",
///     '玩' => "「玩」在里头！",
///     '意' => "「意」在里头！",
///     _ => "这啥玩意…",
/// };
/// // 测试4 使用「属性」同时处理被匹配项和匹配值
/// struct F<I, R>(Box<dyn Fn(I) -> R>);
/// let f = F(Box::new(clj)); // 检测的闭包
/// let f2 = F(Box::new(Box::new)); // 装箱的闭包
/// let v4 = first! {
///     (f.0) => (f2.0);
///     '这' => "「这」在里头！",
///     '啥' => "「啥」在里头！",
///     '玩' => "「玩」在里头！",
///     '意' => "「意」在里头！",
///     _ => "这啥玩意…",
/// };
/// // 展示&断言
/// asserts! {
///     show!(first! {@VALUE (1.cmp) &2}) => std::cmp::Ordering::Less
///     show!(first! {@VALUE (std::any::type_name_of_val) &2}) => "i32"
///     show!(v) => "以一开头！"
///     show!(v2) => "这啥玩意…"
///     show!(v3) => "「这」在里头！".to_string()
///     show!(v4) => Box::new("「这」在里头！")
/// }
/// ```
///
#[macro_export]
macro_rules! first {
    // 第一种方法：直接匹配
    // ! ❌不能在宏中使用不完整的表达式 如单独的`else`等
    { // * 📝←左边的括号只是标注「推荐用括弧」而对实际解析无限定作用
        $guardian_1:expr => $value_1:expr, // ! ←此处必须要逗号分隔表达式，避免解析歧义
        $( $guardian:expr => $value:expr ),*, // ! 逗号仍然必要
        _ => $value_else:expr $(,)? // ←可选的尾后逗号
        // ↑对字面标识「_」无需`$(...)`引用
        // ! ↑但不能把`_ => `标注为可选：local ambiguity when calling macro `first`: multiple parsing options: built-in NTs expr ('value_else') or expr ('guardian').
    } => {
        // 开头
        if ($guardian_1) {
            $value_1
        }
        // 中间
        $(
            else if ($guardian) {
                $value
            }
        )*
        // 结尾
        else {
            $value_else
        }
    };
    // 第二种方法：批量映射
    { // * 📝←左边的括号只是标注「推荐用括弧」而对实际解析无限定作用
        // * ↓🚩此处直接使用令牌树语法，然后在解析时强制使用圆括号解包
        //   * ✨好处：无需考虑里边的内容（兼容任何`f(x)`语法），只要在展开时能拼上就行
        $f_guardian:tt => $f_value:tt;
        $guardian_1:expr => $value_1:expr, // ! ←此处必须要逗号分隔表达式，避免解析歧义
        $( $guardian:expr => $value:expr ),*, // ! 逗号仍然必要
        _ => $value_else:expr $(,)? // ←可选的尾后逗号
        // ↑对字面标识「_」无需`$(...)`引用
        // ! ↑但不能把`_ => `标注为可选：local ambiguity when calling macro `first`: multiple parsing options: built-in NTs expr ('value_else') or expr ('guardian').
    } => {
        // * 📝实际上「在所有出现value的地方处理」就相当于「先得出value，然后处理，再返回」
        first!(@VALUE $f_value
            // 开头
            if (first!(@VALUE $f_guardian $guardian_1)) {
                $value_1
            }
            // 中间
            $(
                else if (first!(@VALUE $f_guardian $guardian)) {
                    $value
                }
            )*
            // 结尾
            else {
                $value_else
            }
        )
    };
    ( @VALUE (_) $value:expr ) => { $value };
    ( @VALUE ($($f:tt)+) $value:expr ) => {
        // f   ( value )
        $($f)+ ($value)
    };
}

/// # `show!`：复现Julia的`@show`
/// * 🎯复刻Julia中常用的宏`@show 表达式`
///   * 相当于Julia`@show(表达式)`，但功能更强大
/// * 📌核心：打印`表达式 = 值`，并（可选地）返回表达式的值
///   * 🚩只有一个表达式⇒计算、打印并返回表达式的值
///   * 🚩多个表达式⇒计算、打印并返回表达式值的元组 | Julia则是返回最后一个值
///   * 🚩一个表达式+尾缀分号⇒计算并打印，**不返回**值
///   * 🚩多个表达式+尾缀分号⇒批量计算并打印，不返回任何值（并且无运行时损耗）
/// * ✅允许尾缀逗号
/// * 📝对于文档测试，必须自包名导入相应的宏以便进行测试
/// * 🔗亦可参考其它实现如[show](https://crates.io/crates/show)
///
/// ## 用例
///
/// ```rust
/// use nar_dev_utils::show;
/// fn see<'a>(v: &'a str, v2: &'a str) -> (&'a str, &'a str) {
///     // 用`show!`打印`v`、`v2`，不返回值
///     show!(&v, &v2;);
///     // 用`show!`打印`v`，并返回其值
///     show!(v, v2)
/// }
/// ```
///
/// 将被转换为
///
/// ```rust
/// fn see<'a>(v: &'a str, v2: &'a str) -> (&'a str, &'a str) {
///     // 用`show!`打印`v`、`v2`，不返回值
///     println!("{} = {:?}", "&v", (&v));
///     println!("{} = {:?}", "&v2", (&v2));
///     // 用`show!`打印`v`，并返回其值
///     (
///         {
///             let value = v;
///             println!("{} = {:?}", "v", value);
///             value
///         },
///         {
///             let value = v2;
///             println!("{} = {:?}", "v2", value);
///             value
///         },
///     )
/// }
/// ```
///
/// 调用`see("我是一个值", "我是另一个值")`将输出
///
/// ```plaintext
/// &v = "我是一个值"
/// &v2 = "我是另一个值"
/// v = "我是一个值"
/// v2 = "我是另一个值"
/// ```
///
/// 并返回`("我是一个值", "我是另一个值")`
#[macro_export]
macro_rules! show {
    // 单参数：求值、打印、返回
    ($e:expr) => {
        {
            // 求值 | 内部赋值
            let value = $e;
            // 打印
            println!("{} = {:?}", stringify!($e), value);
            // 返回 | 上交值（所有权）
            value
        }
    };
    // 单参数but不返回：求值、打印
    // * ↓注意：末尾使用了分号
    ($e:expr;) => {
        // 直接求值并打印
        println!("{} = {:?}", stringify!($e), $e)
    };
    // 多参数&返回：分别求值&打印，输出到元组
    ($($e:expr),+ $(,)?) => {
        // 构造元组
        ( $( show!($e) ),* )
    };
    // 多参数&不返回：分别求值&打印
    ($($e:expr),+ $(,)?;) => {
        // 直接不构造元组
        $( show!($e;) );*;
    };
}

#[allow(clippy::test_attr_in_doctest)] // * 📝告诉Clippy「这只是用来生成单元测试的示例，并非要运行测试」
/// # 辅助用测试宏/批量添加失败测试
///
/// * 可极大减轻添加`should_panic`的代码量
///
/// ! 📝`, $(,)?`这里的「,」代表的不是「分隔表达式」，而是「模式中的`,`」
/// * 故应去除这前边的「,」
///
/// 用法：
///
/// ```rust
/// use nar_dev_utils::fail_tests;
/// // 一般形式：函数名 {代码}
/// fail_tests! {
///     /// 允许文档注释
///     失败测试的函数名 {
///         // 会导致panic的代码
///     }
///     // ... 允许多条
/// }
/// // 亦可：函数名 表达式/语句
/// fail_tests! {
///     /// 允许文档注释
///     失败测试的函数名 if true {panic!("会导致panic的表达式")} else {};
///     // ... 允许多条
/// }
/// fail_tests! {
///     /// 允许文档注释
///     失败测试的函数名 panic!("会导致panic的语句");
///     // ... 允许多条
/// }
/// ```
///
/// ## 用例
///
/// ```rust
/// use nar_dev_utils::fail_tests;
/// fail_tests! {
///     /// 失败测试
///     fail {
///         panic!("这是一个测试")
///     }
///     /// 失败测试二号
///     fail2 {
///         panic!("这是另一个测试")
///     }
/// }
/// ```
///
/// 将被等价转换为
///
/// ```rust
/// /// 失败测试
/// #[test]
/// #[should_panic]
/// fn fail() {
///     panic!("这是一个测试")
/// }
///
/// /// 失败测试二号
/// #[test]
/// #[should_panic]
/// fn fail2() {
///     panic!("这是另一个测试")
/// }
/// ```
///
/// * ✅【2024-03-15 20:15:20】现在借鉴[lazy_static](https://crates.io/crates/lazy_static)包，可以在测试中使用文档字符串了
///   * 📝原理：文档字符串实际上是`#[doc = "一行文本…"]`的语法糖
///   * 📝技法：使用`$(#[$attr:meta])*`匹配元数据，然后原样输出
#[macro_export]
macro_rules! fail_tests {
    // 匹配空块
    {} => {
        // 无操作
    };
    // 匹配代码块
    {$(#[$attr:meta])* $name:ident $code:block $($tail:tt)*} => {
        $(#[$attr])*
        #[test]
        #[should_panic]
        fn $name() {
            $code
        }
        // 尾递归
        fail_tests!($($tail)*);
    };
    // 匹配表达式
    {$(#[$attr:meta])* $name:ident $code:expr; $($tail:tt)*} => {
        $(#[$attr])*
        #[test]
        #[should_panic]
        fn $name() {
            $code; // ← 用分号分隔
        }
        // 尾递归
        fail_tests!($($tail)*);
    };
    // 匹配语句
    {$(#[$attr:meta])* $name:ident $code:stmt; $($tail:tt)*} => {
        $(#[$attr])*
        #[test]
        #[should_panic]
        fn $name() {
            $code
        }
        fail_tests!($($tail)*);
    };
}

/// 用于简化「连续判断相等」的宏
/// * 🎯用于统一
///   * ⚠️缺点：不易定位断言出错的位置（需要靠断言的表达式定位）
/// * 🚩模型：标记树撕咬机
///   * ⚠️缺点：无法一次性展开
///   * 🔗参考：<https://www.bookstack.cn/read/DaseinPhaos-tlborm-chinese/pat-incremental-tt-munchers.md>
///
/// # 用例
///
/// ```rust
/// use nar_dev_utils::asserts;
/// asserts! {
///     1 + 1 > 1, // 判真
///     1 + 1 => 2, // 判等
///     1 + 1 < 3 // 连续
///     1 + 2 < 4, // 判真（与「判等」表达式之间，需要逗号分隔）
///     1 + 2 => 3 // 连续
///     2 + 2 => 4 // 判等（其间无需逗号分隔）
/// }
/// ```
#[macro_export]
macro_rules! asserts {
    // 连续判等逻辑（无需逗号分隔）
    {
        $($left:expr => $right:expr $(,)?)*
    } => {
        $(
            assert_eq!($left, $right, "{} != {}", stringify!($left), stringify!($right));
        )*
    };
    // 连续匹配模式逻辑（无需逗号分隔）
    // * 会和上边的「判等」歧义，所以使用前缀`@`
    // * 📄case：`Some(..)`
    {
        $($left:expr => @ $right:pat $(,)?)*
    } => {
        $(
            assert!(matches!($left, $right), "{} isn't matches {}", stringify!($left), stringify!($right));
        )*
    };
    // 连续判真逻辑（无需逗号分隔）
    {
        $($assertion:expr $(,)?)*
    } => {
        $(
            assert!($assertion, "{} != true", stringify!($assertion));
        )*
    };
    // 新形式/空
    {} => {
        // 无操作
    };
    // 新形式/判真
    {
        $($assertion:expr)*,
        $($tail:tt)*
    } => {
        // 分派到先前情形
        asserts!($($assertion)*);
        // 尾递归
        asserts!($($tail)*)
    };
    // 新形式/判等
    {
        $($left:expr => $right:expr)*,
        $($tail:tt)*
    } => {
        // 分派到先前情形
        asserts!($($left => $right)*);
        // 尾递归
        asserts!($($tail)*)
    };
}

/// 用于简化「连续追加字符串」的宏
/// * 🎯最初用于「字符串格式化」算法中
/// * 🚩用法：`push_str!(要追加入的字符串; 待追加表达式1, 待追加表达式2, ...)`
///
/// ## 用例
///
/// ```rust
/// use nar_dev_utils::push_str;
/// let mut s = String::new();
/// push_str!(
///     &mut s;
///     "这",
///     "是",
///     "可以被",
///     &String::from("连续添加"),
///     "\u{7684}",
/// );
/// assert_eq!(s, "这是可以被连续添加的");
/// ```
#[macro_export]
macro_rules! push_str {
    {$out:expr; $($ex:expr),* $(,)?} => {
        {
            $(
                $out.push_str($ex)
            );*
        }
    };
}

/// 用于将「流式追加」捕捉转换成「固定返回值」
/// * 🎯首次应用于「基于[`String::push_str`]动态追加产生字符串」与「直接返回字符串」的转换中
///   * 📌【2024-03-16 18:05:48】因解析器中应用广泛，目前暂不移除该用法
///
/// # 示例
///
/// 默认用法：生成`String`
///
/// ```rust
/// use nar_dev_utils::catch_flow;
///
/// fn append(out: &mut String) {
///     out.push_str("hello, ");
/// }
///
/// fn append_with(out: &mut String, with: &str) {
///     out.push_str(with);
/// }
///
/// let caught = catch_flow!(append); // 默认用法：使用[`String::new`]生成一个新字串
/// let caught = catch_flow!(caught => append_with; "world!"); // 将捕获结果再次传入，并附加参数
/// assert_eq!(caught, "hello, world!");
/// ```
///
/// 同样可用于非字符串变量：
///
/// ```rust
/// use nar_dev_utils::catch_flow;
///
/// fn add_one(n: &mut usize) {
///     *n += 1;
/// }
///
/// let caught = catch_flow!(0 => add_one);
/// assert_eq!(caught, 1);
/// ```
#[macro_export]
macro_rules! catch_flow {
    // 原始语法：`(对象.方法; 其它参数)`
    // * 📝现在直接转发到新实现
    // * 📌
    ( $($path:ident).+ $(; $($tail:tt)*)? ) => {
        catch_flow!({String::new()} => {$($path).+} $(; $($tail)*)? )
    };
    // 原始语法の扩展：`(对象.方法; 其它参数)`
    // * 📝现在直接转发到新实现
    ( $value:expr => $($path:ident).+ $(; $($tail:tt)*)? ) => {
        catch_flow!({$value} => {$($path).+} $(; $($tail)*)? )
    };
    // 新语法：`([ 对象 ] => [流式追加函数] ; 其它参数)`
    ( { $($value:tt)+ } => { $($f:tt)+ } ; $($arg:tt)* ) => {
        {
            let mut target = $($value)+;
            $($f)+ (&mut target, $($arg)*);
            target
        }
    };
    // 新语法简写：`([ 对象 ] => [流式追加函数] ; 其它参数)`
    ( { $($value:tt)+ } => { $($f:tt)+ } ) => {
        {
            let mut target = $($value)+;
            $($f)+ (&mut target);
            target
        }
    };
}

/// 更通用的「函数参数张量展开」宏
/// * 🎯用于最终版简化一系列「笛卡尔积式组合调用」
/// * 🚩【2024-03-09 15:01:24】与「函数参数矩阵展开」宏合并
///   * 📌后者（矩阵）可看作「二维张量」
/// * ⚠️对「内部转换」`@inner`的规定性约束：
///   * 🚩统一使用方括号，避免「圆括号→表达式值」的歧义
///   * 🚩统一使用逗号分隔（强制尾后逗号），避免「连续圆括号→函数调用」的歧义
///
/// # Example
///
/// ```rust
/// use nar_dev_utils::f_tensor;
/// fn add(a: i32, b: i32) -> i32 {
///     a + b
/// }
/// fn add3(a: i32, b: i32, c: i32) -> i32 {
///     a + b + c
/// }
///
///  // fallback情况
/// let m = f_tensor!(@inner [add] [1, 2,]);
/// assert_eq!(m, 3);
///
///  // fallback情况 2
/// let m = f_tensor!(@inner [add] [1, 2,] []);
/// assert_eq!(m, 3);
///
///  // 正常情况
/// let m1 = f_tensor![add [1 2] [3 4 5]];
/// let m2 = f_tensor![add3; 1 2; 3 4; 5 6];
/// // 📌↓此处对「括号表达式」可用逗号明确分隔，以避免「函数调用」歧义
/// let m3 = f_tensor![add3 [(2-1), (1+1)] [3 4] [5 6]];
///
/// assert_eq!(m1, [[4, 5, 6], [5, 6, 7]]);
/// assert_eq!(
///     m2,
///     // ↓展开结果
///     [
///         [[1 + 3 + 5, 1 + 3 + 6], [1 + 4 + 5, 1 + 4 + 6]],
///         [[2 + 3 + 5, 2 + 3 + 6], [2 + 4 + 5, 2 + 4 + 6]],
///     ]
/// );
/// // ↓计算结果
/// assert_eq!(m3, [[[9, 10], [10, 11]], [[10, 11], [11, 12]],]);
/// ```
///
/// # Experiences
///
/// * 📝使用「前缀特殊标识符」控制宏匹配时的分派路径
///   * 💭此举特别像Julia的多分派系统
/// * 📝涉及「嵌套笛卡尔积展开」时，把其它变量都变成一个维度，在一次调用中只展开一个维度
///   * 🚩源自GitHub Issue的方法
///     * 1 先使用「数组」之类的包装成一个令牌树（tt）
///     * 2 展开另一个维度
///     * 3 再将原先包装的维度解包
///
/// # References
///
/// * 🔗宏小册「使用`@`标识子分派」<https://www.bookstack.cn/read/DaseinPhaos-tlborm-chinese/aeg-ook.md>
/// * 🔗开发者论坛：<https://users.rust-lang.org/t/why-is-the-innermost-meta-variable-expansion-impacted-by-the-outmost-one/99099/4>
/// * 🔗GitHub Issue：<https://github.com/rust-lang/rust/issues/96184>
#[macro_export]
macro_rules! f_tensor {
    // 入口/空格分号形式 | 可选逗号进行无歧义分隔
    // * f_tensor![f; 1 2 3; 4 5 6]
    [
        // 要被调用的函数（标识符）
        $($path:ident).+;
        // 参数的表达式序列
        $($($arg:expr $(,)?)+);+ $(;)?
    ] => {
        // * 0 包装后边的参数到数组（这样后续可以用tt替代）
        f_tensor![
            $($path).* $( [ $($arg)+ ] )+
        ]
    };
    // 入口/数组形式（内外桥梁） | 可选逗号进行无歧义分隔
    // * `f_tensor![f [1 2 3] [4 5 6]]` => ``f_tensor![[f] [] [[1, 2, 3,] [4, 5, 6,]]]``
    [
        // 要被调用的函数（标识符序列）
        $($path:ident).+
        // 参数的表达式序列
        $( [ $($arg:expr $(,)? )+ ] )+
    ] => {
        // * 1 开始解析
        f_tensor![
            // 加上标识符
            @inner
            // 将「被调用函数」打包（以便支持如`self.add`的表达形式）
            [$($path).+]
            // 空参数集（未开始填充）
            []
            // 包装：`([参数集1], [参数集2] ...)`
            [ $( [ $($arg,)+ ], )+ ]
        ]
    };
    // 【内部】「纯参数」fallback情况
    // * `f_tensor![[f] [1, 2, 3,]]` => `f(1, 2, 3)`
    [
        // 内部标识符
        @inner
        // 要被调用的函数（已作`[fn]`包装，此处解包）
        [ $($f:tt)+ ]
        // 只有一个表达式序列
        [ $($arg:expr,)+ ]
    ] => {
        // 直接解包
        $($f)* ($($arg),+)
    };
    // 【内部】参数+空括号 情况
    // * `f_tensor![[f] [1, 2, 3,] []]` => `f_tensor![[f] [1, 2, 3,]]`
    [
        // 内部标识符
        @inner
        // 要被调用的函数（已作`[fn]`包装）
        $f:tt
        // 表达式序列
        $args:tt
        // 空括号
        []
    ] => {
        // 去掉空括号
        f_tensor![@inner $f $args]
    };
    // 【内部】参数+参数 情况
    // * `f_tensor![[f] [1, 2, 3,] [[x1, x2, ...x,] ...tail]]` => `...f_tensor![[f] [1, 2, 3, x,] [...tail]]`
    [
        // 内部标识符
        @inner
        // 要被调用的函数（已作`[fn]`包装）
        $f:tt
        // 表达式序列（此处延迟解包，留给后边的`append`）
        $args_head:tt
        // [[参数头...] 其它参数...]
        [ [ $($x:expr,)+ ], $($tail:tt)* ]
    ] => {
        // * 解构，留给专门的函数进行展开（因为x和tail不能同时展开）
        f_tensor![
            // 调用新函数
            @inner_expand
            // 直接传递被调用者
            $f
            // 直接传递表达式序列（后续「展开」「追加」要一次完成）
            $args_head
            // 提取x序列
            [ $($x,)+ ]
            // 去头 | 先展开tail
            [ $($tail)* ]
        ]
    };
    // * 【内部】工具分派/展开
    [
        // 内部标识符
        @inner_expand
        // 要被调用的函数（已作`[fn]`包装）
        $f:tt
        // 表达式序列（此处延迟解包，留给后边的`append`）
        $args_head:tt
        // 提取的x序列（预备展开）
        [ $($x:expr,)+ ]
        // (其它参数...)
        $other_args:tt
    ] => {
        // * 开始【展开】一个维度
        [
            $(f_tensor![
                // 在展开之后专门追加
                @inner_append
                // 直接传递被调用者
                $f
                // 表达式序列原样传递
                $args_head
                // ! 这里不能「宏套宏」：「表示『追加』的宏调用」被认成表达式了
                // f_tensor!( @append $args_head $x )
                [ $x ] // 提取出来的x
                // 留下的尾部序列
                $other_args
            ]),+
        ]
    };
    // * 【内部】工具分派/追加
    [
        // 内部标识符
        @inner_append
        // 要被调用的函数（已作`[fn]`包装）
        $f:tt
        // 表达式序列（此处解包）
        [ $($arg_head:expr,)* ]
        // 提取的x
        [ $x:expr ]
        // (其它参数...)
        $other_args:tt
    ] => {
        f_tensor![
            // 回到原先的展开进程
            @inner
            // 直接传递被调用者
            $f
            // 展开的参数【追加】到函数参数序列中
            [ $($arg_head,)* $x, ]
            // 留下的尾部序列
            $other_args
        ]
    };
}

/// 平行将参数填充进函数
/// * 📄形式：`f_parallel![add3; 1 2 3; 4 5 6]` => `[add3(1, 2, 3), add3(4, 5, 6)]`
///
/// # Example
///
/// ```rust
/// use nar_dev_utils::f_parallel;
/// fn add3(a: i32, b: i32, c: i32) -> i32 {
///     a + b + c
/// }
/// let m = f_parallel![
///     add3;
///     1 2 3; // add3(1, 2, 3)
///     4 5 6; // add3(4, 5, 6)
///     7, (8) 9; // add3(7, 8, 9) // ! 📌此处使用逗号避免「调用歧义」`7(8)`
/// ];
/// assert_eq!(m, [6, 15, 24]);
/// ```
///
#[macro_export]
macro_rules! f_parallel {
    // 入口/空格分号形式 | 可选逗号进行无歧义分隔
    [
        // 要被调用的函数（标识符）
        $($path:ident).+;
        // 参数的表达式序列 // ! ↓此处必须限制为「+」，不然无法实现「尾后分号」（会引发解析歧义）
        $( $( $arg:expr $(,)? )+ );* $(;)?
    ] => {
        // * 🚩先封装好：`f_parallel![add3; 1 2 3; 4 5 6]` => `f_parallel![@inner [add3] [1, 2, 3,] [4, 5, 6,]]`
        f_parallel![
            // 内部标识符
            @inner
            // 要被调用的函数（标识符序列）
            [$($path).+]
            // 参数的表达式序列
            $( [ $($arg,)* ] )*
        ]
    };
    // 【内部】先展开参数
    // * `f_parallel![@inner [add3] [1, 2, 3,] [4, 5, 6,]]` => `f_parallel![@inner [add3] [1, 2, 3,] [4, 5, 6,]]`
    [
        @inner
        // 要被调用的函数（标识符）
        $f:tt
        // 参数的表达式序列的序列
        $( [ $($arg:expr,)* ] )*
    ] => {
        [
            $(f_parallel![
                // 内部标识符
                @inner_expand
                // 要被调用的函数（已作`[fn]`包装）
                $f
                // 参数的表达式序列
                [ $($arg,)+ ]
            ]),*
        ]
    };
    // 【内部】再展开函数表达式
    [
        @inner_expand
        // 要被调用的函数（标识符）
        [ $($f:tt)* ]
        // 参数的表达式序列
        [ $($arg:expr,)* ]
    ] => {
        $($f)* ($($arg),+)
    };
}

/// 简化「if 条件 {return 值;}」的控制流
/// * 📄形式：`if_return![a == 1 => 2]` => `if a == 1 {return 2;}`
///
/// # Examples
///
/// ```rust
/// use nar_dev_utils::if_return;
/// fn starts_at(text: &str, prefix: &str) -> Option<usize> {
///     // 截断式返回示例：多分支
///     if_return! {
///        prefix.is_empty() => Some(0)
///        text.starts_with(prefix) => Some(0)
///     }
///
///     let mut i = 0;
///     let max_i = text.len() - prefix.len();
///     while i <= max_i {
///         // 截断式返回示例：单分支 | 三行变一行
///         if_return! { prefix == &text[i..(i + prefix.len())] => Some(i) }
///         i += 1;
///     }
///
///     None
/// }
///
/// assert_eq!(starts_at("hello", ""), Some(0));
/// assert_eq!(starts_at("hello", "llo"), Some(2));
/// assert_eq!(starts_at("hello", "help"), None);
/// ```
///
/// ```rust
/// use nar_dev_utils::if_return;
/// fn raise_the_bar(num: usize, bar: &mut usize) {
///     #![allow(clippy::unused_unit)]
///     *bar = 0;
///     // 截断式返回示例：隐式指定返回值（单元类型`()`）
///     if_return! { num <= *bar }
///     println!("{num} is greater than {bar}");
///
///     *bar = 1;
///     // 截断式返回示例：上述「隐式返回」与此处「显式返回」等价
///     if_return! { num <= *bar => () }
///     println!("{num} is greater than {bar}");
///     *bar = 2;
/// }
///
/// let mut num = 0;
/// let mut bar = 0;
/// raise_the_bar(num, &mut bar);
/// assert_eq!(bar, 0);
///
/// num = 1;
/// raise_the_bar(num, &mut bar);
/// assert_eq!(bar, 1);
///
/// num = 2;
/// raise_the_bar(num, &mut bar);
/// assert_eq!(bar, 2);
/// ```
#[macro_export]
macro_rules! if_return {
    // 特殊优化/单条：直接返回表达式
    {
        $condition:expr $(=> $return_value:expr)?
    } => {
        if $condition {
            return $($return_value)?;
        }
    };
    // 推广情况/多条：使用代码块分别包裹
    // * 📝嵌套展开并非不可，只是「多对多」更复杂
    //   * 【2024-03-16 21:55:22】目前更多要靠自己试
    {
        $($condition:expr $(=> $return_value:expr)?)*
    } => {
        $(
            {if $condition {
                return $($return_value)?;
            }};
        )*
    };
}

/// 批量封装：在限定的特性（feature）下，导入并重新导出模块
/// * 🎯用于简化重复的`#[cfg(feature = XXX)]`以及`pub mod`、`pub use`逻辑
/// * ⚠️已知问题：**无法以此覆盖【内部导出了宏】的模块**
///   * 🔗问题参考：<https://github.com/rust-lang/rust/pull/52234>
/// * 🚩【2024-03-18 22:04:24】出于对调用者的考虑，此处对模块及其符号都选择「公开导出」
#[macro_export]
macro_rules! feature_pub_mod_and_reexport {
    // ! 弃用「单名称，自动转换并填充标识符」的做法
    // * ❌【2024-03-18 21:17:12】暂时还没找到「标识符⇒同名字符串」的映射
    //   * 📝`stringify`不行
    // ($name:ident) => {
    //     feature_pub_and_reuse! {
    //         stringify!($name) => $name
    //     }
    // };
    // 默认 | 导出内部模块
    { $( $feature_name:literal => $mod_name:ident )* } => {
        $(
            #[cfg(feature = $feature_name)]
            pub mod $mod_name; // ! 默认公开（允许细分一层路径以解决重名问题）
            #[cfg(feature = $feature_name)]
            pub use $mod_name::*; // ! 公开
        )*
    };
}

/// 批量封装：导入并重新导出模块
/// * 🎯用于简化重复的`pub mod`、`pub use`逻辑
/// * ⚠️已知问题：**无法以此覆盖【内部导出了宏】的模块**
///   * 🔗问题参考：<https://github.com/rust-lang/rust/pull/52234>
/// * 🚩【2024-03-18 22:04:24】出于对调用者的考虑，此处对模块及其符号都选择「公开导出」
#[macro_export]
macro_rules! pub_mod_and_pub_use {
    // 默认
    { $( $mod_name:ident )* } => {
        $(
            pub mod $mod_name; // ! 公开
            pub use $mod_name::*; // ! 公开
        )*
    };
}

/// 批量封装：导入并重新导出模块
/// * 🎯用于简化重复的`mod`、`pub use`逻辑
/// * ⚠️已知问题：**无法以此覆盖【内部导出了宏】的模块**
///   * 📄参考：[`pub_mod_and_reexport`]
#[macro_export]
macro_rules! mod_and_pub_use {
    // 默认
    { $( $mod_name:ident )* } => {
        $(
            mod $mod_name; // ! 不公开
            pub use $mod_name::*; // ! 公开
        )*
    };
}

/// # **pipe!**
///
/// 一个实用、强大而高效的「管道」宏，允许带任意数量插值的任意函数调用
/// * 🎯用以实现类似Julia `@pipe`的「管道处理」效果
/// * 📌使用占位符`_`进行插值
///   * ✅允许多重插值——但会复制整个表达式
/// * ✨部分灵感来自Julia的[**Pipe.jl**](https://github.com/oxinabox/Pipe.jl)
///   * 📄其中的宏`@pipe`有类似的效果
/// * ⚠️【2024-03-26 00:16:36】目前对「完全限定语法」尚未有良好支持
///   * 📄`Vec::<usize>::new`会因「大于/小于 符号」失效
///
/// ## 📄示例语法
///
/// ```rust
/// use nar_dev_utils::{pipe, asserts};
/// fn f1(x: i32) -> i32 { x + 1 }
/// fn f2(x: i32, y: i32) -> i32 { x + y }
/// fn f3(x: i32, y: i32) -> i32 { x - y }
///
/// let v = 1;
/// let x = 2;
/// let y = 3;
/// // (((x + 1) + 2) - 3)
/// let piped = pipe! { v => f1 => f2(x, _) => (f3)(_, y) => f2(_, _) };
/// let normal = f2(f3(f2(x, f1(v)), y), f3(f2(x, f1(v)), y));
/// asserts!{
///     piped => normal,
///     piped => 2,
/// };
/// ```
///
/// ## 🚩内部实现
///
/// * 📌递归模型：标签树撕咬机 + 多分派状态机
///   * 总体流程：用户输入 ⇒ 内部输入 ⇒ 单次管道返回值 ⇒ 尾递归回代
/// * 📌对「被管道的函数」的语法支持
///   * `标识符`
///   * `#{前缀}`
///   * `.属性`
///   * `.方法(..参数)`
///   * `(表达式)`
///   * `模块::函数`
///   * `[对象.方法]`
///
/// ## ✅规模化测试
///
/// ```rust
/// use nar_dev_utils::{pipe, asserts};
/// mod m {
///     pub fn add_one(x: i32) -> i32 {
///         x + 1
///     }
///     pub fn tri_add(x: i32, y: i32, z: i32) -> i32 {
///         x + y + z
///     }
/// }
/// use m::add_one;
///
/// asserts! {
///
///     // 内部情形
///     pipe! {@CALL [add_one] [1]} => 2,
///     pipe! {@CALL [i32::checked_add] [1] => [_, 2]} => Some(3),
///     pipe! {@CALL [m::tri_add] [1] => [_, 2, 3]} => 6,
///     pipe! {@CALL [m::tri_add] [2] => [1, _, 3]} => 6,
///     pipe! {@CALL [m::tri_add] [3] => [1, 2, _]} => 6,
///
///     // 平凡情况：单个值 //
///     pipe! { 1 } => 1,
///     pipe! { 1 + 1 } => 2,
///     pipe! { add_one(1) } => 2,
///     pipe! { match 1 { 1 => 2, _ => 0 } } => 2,
///
///     // 实用辅助：借用、访问 //
///
///     // 测试`self.method`
///     pipe! {
///         "I can be turned into a &str!"
///         => String::from // 转换成字符串
///         => #{&} // 加上前缀`&`转换为`&String`
///         => .as_str() // 转换为`&str`
///         => .to_lowercase() // 全小写
///     } => "i can be turned into a &str!",
///     // 测试`self.field`、`&mut`
///     {
///         let mut s_0 = ("Hello, ".to_string(), 0);
///         pipe! {
///             s_0
///             => .0 // 获取内部的`String`
///             => #{&mut} // 获取`&mut String`
///             => .push_str("pipe!") // 追加字符串
///         };
///         s_0.0
///     } => "Hello, pipe!",
///
///     // 最简单的情况：单函数 //
///
///     // 直接使用标识符
///     pipe! {1 => add_one} => 2,
///     // 模块路径
///     pipe! {1 => m::add_one} => 2,
///     // 关联函数
///     pipe! {&vec![1] => Vec::len} => 1,
///     // 内部使用闭包的表达式
///     pipe! {1 => (|x| x + 1)} => 2,
///     // 对象的方法
///     pipe! {1 => [0_i32.min]} => 0,
///
///     // 复杂情况：函数插值 //
///
///     // 单重插值语法
///     pipe! {1 => i32::checked_add(_, 1)} => Some(2),
///     pipe! {1 => i32::min(0, _)} => 0,
///     pipe! {1 => (|x, y| x + y)(_, 1)} => 2,
///     pipe! {1 => (|x, y, z| x * y * z)(1, _, 2)} => 2,
///
///     // 多重插值语法 | 直接拷贝表达式
///     pipe! { @INSERT [usize::checked_add] [1] => [2, 3] } => Some(5)
///     pipe! { @INSERT [m::tri_add] [1] => [_, _, 3] } => 5
///     pipe! { @INSERT [m::tri_add] [2] => [1, _, _] } => 5
///     pipe! { @INSERT [m::tri_add] [3] => [_, 2, _] } => 8
///     pipe! { @INSERT [usize::checked_add] [0] => [_, _] } => Some(0)
///     pipe! { @CALL [usize::checked_add] [0] => [_, _] } => Some(0)
///
///     // 复杂情况：多函数链路 //
///
///     // 直接使用标识符
///     pipe! {1 => add_one => add_one} => 3,
///     // 模块路径
///     pipe! {1 => m::add_one => m::add_one} => 3,
///     // 关联函数
///     pipe! {&vec![1] => Vec::len => usize::checked_add(_, 1)} => Some(2),
///     // 内部使用闭包的表达式
///     pipe! {1 => (|x| x + 1) => (|x| x + 1)} => 3,
///     // 对象的方法
///     pipe! {1 => [0_i32.min] => [1_i32.max]} => 1,
///
///     // 大 杂 烩 //
///     pipe! {
///         // 最初的值
///         &vec![1]
///         // 关联函数
///         => Vec::len // 1
///         // 内部使用闭包的表达式
///         => (|x:usize| x as i32) // 转换类型
///         => (|x| x + 1) // 2
///         // 内部使用闭包的表达式（带参数）
///         => (|x, y| x - y)(_, 1) // 1
///         // 直接使用标识符
///         => add_one // 2
///         // 模块路径
///         => m::add_one // 3
///         // 关联函数（带参数）
///         => i32::checked_sub(_, 1) // Some(2)
///         => Option::unwrap // 2
///         // 对象的方法
///         => [1_i32.min] // 1
///         => [(-1_i32).max] // 1
///     } => 1
/// }
/// ```
#[macro_export]
macro_rules! pipe {
    // 单函数展开
    { @CALL [ $($f:tt)* ] [ $($value:expr),* ] } => { $($f)* ( $($value),* ) };

    // 插值语法
    // * ❌二重插入宏展开结果 不可取
    {
        @CALL
        $f:tt
        $value:tt => $args:tt
    } => {
        pipe! {
            @INSERT
            $f
            $value => $args
        }
    };
    // ! ❌【2024-03-26 00:13:18】↓弃用：nested宏调用失败（宏调用本身被解释成了token）
    // {
    //     @CALL $f:tt
    //     $value:tt => $args:tt
    // } => {
    //     pipe! {
    //         @CALL
    //         $f
    //         pipe! {
    //             @INSERT
    //             $value => $args
    //         }
    //     }
    // }; // ←此处可能有尾后逗号

    //
    { // 终态：插入完成
        @INSERT
        [ $($f:tt)* ]
        $_value:tt =>
        [
            $( $values:expr ),*
            $(,)?
        ]
    } => { $($f)* ( $( $values ),* ) };
    { // 中态：不断插入
        @INSERT
        $f:tt
        [ $value:expr ] =>
        [
            $( $value_past:expr, )*
            _
            $( $tail:tt )*
        ]
    } => {
        pipe! {
            @INSERT
            $f
            [ $value ] =>
            [
                $( $value_past, )*
                $value
                $( $tail )*
            ]
        }
    };

    // 简单情形：表达式/令牌树 | ✅实验用，现已弃用
    // { $value:expr => [$($dot_path:tt).+] } => {pipe!{ @CALL [$($dot_path).+] [($value)] }};
    // { $value:expr => $f:ident } => {pipe!{ @CALL [$f] [($value)] }};
    // { $value:expr => $f:path } => {pipe!{ @CALL [$f] [($value)] }};
    // { $value:expr => ($f:expr) } => {pipe!{ @CALL [($f)] [($value)] }};
    // { $value:expr => $f:tt } => {pipe!{ @CALL [$f] [($value)] }}; // ! ❌【2024-03-25 23:01:46】不能启用`tt`：会把`[$dot_path]`搞歧义

    // pipe! {@CALL[[0_i32.min]][(1)]}
    // [0_i32.min]((1))

    // 递归出口：所有值都折叠到单个表达式
    { $value:expr } => { $value };
    // 用户入口：单个管道方法/附加前缀`&self`
    {
        $value:expr =>
        #{ $($prefix:tt)* }
        $( => $($tail:tt)*)?
    } => {
        pipe! {
            $($prefix)* $value
            $( => $($tail)*)?
        }
    };
    // 用户入口：单个管道方法/点号语法`self.method`/`self.field`
    {
        $value:expr =>
        . $key:tt $( ( $($param:tt)* ) )?
        $( => $($tail:tt)*)?
    } => {
        pipe! {
            $value.$key $( ( $($param)* ) )?
            $( => $($tail)*)?
        }
    };
    // 用户入口：单个管道方法/点路径`self.method`
    {
        $value:expr =>
        [ $($dot_path:tt).+ ] $( ( $($param:tt)* ) )?
        $( => $($tail:tt)*)?
    } => {
        pipe! {
            pipe! {
                @CALL
                [ $($dot_path).+ ]
                [ ($value) ] $( => [ $($param)* ] )?
            }
            $( => $($tail)*)?
        }
    };
    // 用户入口：单个管道方法/模块路径`module::function`
    {
        $value:expr =>
        $($p:tt)::+           $( ( $($param:tt)* ) )?
        $( => $($tail:tt)*)?
    } => {
        pipe! {
            pipe! {
                @CALL
                [ $($p)::+ ]
                [ ($value) ] $( => [ $($param)* ] )?
            }
            $( => $($tail)*)?
        }
    }; // ! ❌不能直接用`path`匹配：后边无法跟`(...)`
    // 用户入口：单个管道方法/单个表达式`(expr1 + expr2)`
    {
        $value:expr =>
        ($f:expr)             $( ( $($param:tt)* ) )?
        $( => $($tail:tt)*)?
    } => {
        pipe!{
            pipe! {
                @CALL
                [ ($f) ]
                [ ($value) ] $( => [ $($param)* ] )?
            }
            $( => $($tail)*)?
        }
    };
    // { $value:expr => $f:tt } => {pipe!{ @CALL [$f] [($value)] }}; // ! ❌【2024-03-25 23:01:46】不能启用`tt`：会把`[$dot_path]`搞歧义
}
