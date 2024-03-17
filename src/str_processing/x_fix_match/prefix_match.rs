//! 与「前缀匹配」有关的工具结构与算法
//! * 🎯最初用于字符串parser
//! * 🎯存储专用的「前缀匹配」实现
//! * 📌【2024-03-17 14:18:01】现在从「后缀匹配」子模块迁移而来

use super::traits::*;

/// 「前缀条目」
/// * 🎯统一表达`(关联内容, 前缀)`的二元组
type PrefixTerm<T, XFix = Prefix> = (XFix, T);

/// 前缀配对字典
/// * 🚩具体逻辑：
///   * 【倒序】维护一个有一定顺序、不重复的[`String`]数组
///   * 使用「二元组」直接在数组内建立「字符串⇒其它元素」的关联
#[derive(Debug, Clone)]
pub struct PrefixMatchDictPair<T> {
    /// * 🎯此处设计成`(关联内容, 前缀)`元组顺序，纯粹为了代码上`("<", ">")`对称
    pub(super) prefixes: Vec<PrefixTerm<T>>,
}

/// 实现「默认构造函数」
/// * 🚩通过「初始化空数组」完成
impl<T> Default for PrefixMatchDictPair<T> {
    fn default() -> Self {
        Self {
            prefixes: Vec::new(),
        }
    }
}

/// 通过宏快捷构造「前缀配对字典」
/// * 📌格式：「前 => 后」
#[macro_export]
macro_rules! prefix_match_dict_pair {
    // 转换其中的值 | 静态字串⇒动态字串 自动`into`
    (@value $v:literal) => {
        $v.into()
    };
    // 转换其中的值 | 表达式⇒直接加入
    (@value $v:expr) => {
        $v
    };
    // 统一的表 | 自面量也是一种表达式
    [$($prefix:expr => $item:expr $(,)?)*] => {{
        let mut d = $crate::PrefixMatchDictPair::default();
        $(
            d.insert((
                prefix_match_dict_pair!(@value $prefix),
                prefix_match_dict_pair!(@value $item),
            ));
        )*
        d
    }};
}

/// 实现专用方法
impl<T> PrefixMatchDictPair<T> {
    /// 构造函数
    /// * ⚠️实际上不推荐从所谓「迭代器」直接创建数组：**难以预先排序**
    /// * 🚩现在采用「先新建空值，然后逐个添加」来实现
    ///   * 📌复杂度：∑ 1 log 1 ~ n log n
    /// * 📌格式：`条目=(其它元素, 前缀)`
    pub fn new(prefixes: impl IntoIterator<Item = PrefixTerm<T, impl Into<Prefix>>>) -> Self {
        // ! ❌【2024-03-17 16:42:39】不再尝试提取构造函数
        // 使用`default`创建一个空值
        let mut dict = Self::default();
        // 逐个添加
        for term_to_into in prefixes.into_iter() {
            // 针对`Into`做一个「转换再插入」
            // ! 仍然无法使用`get_associated_from_term`：这时候的`term_to_into`还不是`term`
            dict.insert(Self::new_prefix_term(term_to_into.0.into(), term_to_into.1));
        }
        // 返回
        dict
    }

    /// 前缀条目→前缀（引用）
    /// * 📌是`&String`而非`&str`，真正引用到前缀实例本身
    /// * ⚠️【2024-03-17 16:04:52】目前暂无法提取至特征
    ///   * 📌原因：对「词缀匹配字典」无法容忍「(自身, 自身)」参数
    #[inline(always)]
    pub fn prefix_ref_of(term: &PrefixTerm<T>) -> &Prefix {
        &term.0
    }

    /// 用于从一个「前缀条目」中获取「关联内容」
    /// * ⚠️【2024-03-17 13:05:43】目前暂无法提取至特征
    ///   * 📌原因：对「词缀匹配字典」无法容忍「(自身, 自身)」参数
    #[inline(always)]
    pub fn get_associated_from_term(term: &PrefixTerm<T>) -> &T {
        &term.1
    }

    /// 从「前缀」与「关联内容」组装「前缀条目」
    /// * ⚠️【2024-03-17 13:05:43】目前暂无法提取至特征
    ///   * 📌原因：对「词缀匹配字典」无法容忍「(自身, 自身)」参数
    #[inline(always)]
    pub fn new_prefix_term(prefix: Prefix, associated: T) -> PrefixTerm<T> {
        (prefix, associated)
    }

    /// 判断「是否已有一个前缀」
    /// * 📌直接使用自身的「搜索」功能
    /// * ⚠️因涉及【涉及内部字段】的[`Self::search`]方法，无法提取至特征
    #[inline(always)]
    pub fn has(&self, prefix: &Prefix) -> bool {
        // * 🚩查找「ok」证明「能找到」
        self.search(prefix).is_ok()
    }

    /// 插入一个条目
    /// * 🚩调用经分派的「查找」方法
    /// * 🚩返回「插入后元素位置的索引」
    ///   * 🎯相比「是否插入」提供了「插入后的元素」的信息
    ///   * 📝**若返回「插入后元素的引用」，则会导致字段连带自身被持续借用**
    ///     * ❗因此导致「借了不还」的生命周期问题
    ///     * ✨此时还不如返回索引，让调用者根据索引调用
    ///     * 📌关键注意点：**返回引用要慎重**
    ///   * 📝✨一个对「生命周期问题」非常实用的**错误追溯方法**：内联
    ///     * 📌核心原理「代码多态」：函数内联后的代码，只需稍加修改其中的`return`，就能直接替换调用处的表达式
    ///       * 📄依据：块作用域、块表达式——语句块作为一个单独的作用域，且可以被求值
    ///     * 🚩借助编辑器的「代码inline」功能，不断展开所调用的代码，直到无法展开
    ///       * 📌最后一般会追溯到「字段访问」「标准库函数」等基础操作
    ///       * ✅此时便可从全局角度审视代码
    pub fn insert(&mut self, term: PrefixTerm<T>) -> Option<usize> {
        // 只有在「查找失败」时进行插入
        if let Err(index) = self.search(Self::get_prefix_from_term(&term)) {
            // 直接调用数组的「插入」方法
            self.prefixes.insert(index, term);
            // * 📌【2024-03-17 19:08:37】插入后还是那个位置，所以直接返回（而非再借用）
            return Some(index);
        }
        // 插入失败⇒失败
        None
    }

    /// （前前缀无关）以特殊顺序迭代「词缀」
    /// * 🎯统一「前前缀匹配」的迭代逻辑
    /// * 🚩总是按照「字典顺序」倒序遍历：**长度从长到短**
    #[inline(always)]
    pub fn iter_terms<'a>(&'a self) -> impl Iterator<Item = &PrefixTerm<T>> + 'a
    where
        String: 'a,
    {
        // ! 【2024-03-17 12:34:00】此处因为「原先就以倒序存储」所以直接顺序遍历
        self.prefixes.iter()
    }

    /// 搜索
    /// * 🎯构造可方便替换的「查找」逻辑
    /// * 🚩找到⇒位置，没找到⇒应该插入的位置
    ///   * 📌这个「应该插入的位置」需要**让插入后「从大到小排列」**
    ///   * 📌亦即【渐进式构建】「前缀从长到短的数组」
    /// * 📌【2024-03-17 12:13:14】实际上对「待插入的条目」只需要其前缀信息
    ///   * 故直接根据前缀寻找
    /// * ⚠️因涉及「内部数组」所以【无法提取至通用特征】
    #[inline(always)]
    pub fn search(&self, prefix: &PrefixStr) -> Result<usize, usize> {
        super::search_by(&self.prefixes, &prefix, |prefix, existed| {
            // ! ⚠️不要在此添加`.reverse()`：限定在`cmp之内解决
            // * 📌保证插入后「比自己大的 > 自己 > 已存在」
            Self::cmp_prefix(existed, prefix)
        })
    }
}

/// 实现「前缀匹配」逻辑
impl<T> PrefixMatch<PrefixTerm<T>> for PrefixMatchDictPair<T> {
    // 下面的方法直接进行「特化重定向」处理 //
    fn get_prefix_from_term(term: &PrefixTerm<T>) -> &PrefixStr {
        // 返回的「前缀引用」【自动转换】为静态引用 | &String -> &str
        Self::prefix_ref_of(term)
    }
    fn prefix_terms<'a>(&'a self) -> impl Iterator<Item = &'a PrefixTerm<T>> + 'a
    where
        PrefixTerm<T>: 'a,
    {
        self.iter_terms()
    }
}

/// 单元测试/前缀匹配
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{show, test_match_prefix};

    /// 测试/边缘
    #[test]
    fn test_edge() {
        // 构造测试用例
        let d: PrefixMatchDictPair<String> = prefix_match_dict_pair!(
            "" => "0" // 空值fallback
            "a" => "1"
            "aa" => "2"
            "aaa" => "3"
        );
        show!(&d);
        // 开始匹配
        test_match_prefix! {
            d;
            // 完全匹配
            "a" => Some("1")
            "aa" => Some("2")
            "aaa" => Some("3")
            // 范围内情况
            "a_" => Some("1")
            "aa_" => Some("2")
            "aaa_" => Some("3")
            // 空值fallback
            "" => Some("0")
            "b" => Some("0")
        }
    }

    /// 测试/实战
    #[test]
    fn test_prefix_match_pairs() {
        // 测试「括弧匹配」
        let d: PrefixMatchDictPair<String> = prefix_match_dict_pair!(
            "(" => ")"
            "[" => "]"
            "{" => "}"
            "<" => ">"
        );
        show!(&d);
        // 测试前缀匹配
        test_match_prefix! {
            d;
            // 范围内情况
            r"(A, B, C)" => Some(")")
            r"[A, B, C]" => Some("]")
            r"{A, B, C}" => Some("}")
            r"<A, B, C>" => Some(">")
            // 无效情况
            "word" => None
        }

        // 测试「预算值」匹配
        let d: PrefixMatchDictPair<String> = prefix_match_dict_pair!(
            // 预算值 //
            "$" => "$"
            r"\$" => r"\$"
            "预" => "算"
        );
        show!(&d);
        // 测试后缀匹配
        test_match_prefix! {
            d;
            // 范围内情况 | ❗后缀⇒前缀 //
            // ASCII
            "$0.4;0.4;0.4$ <A-->B>." => Some("$")
            // LaTeX
            r"\$0.4;0.4;0.4\$ \left<A \rightarrow  B\right>." => Some(r"\$")
            // 漢文
            "预0.4、0.4、0.4算「A是B」。" => Some("算")
            // 无效情况 //
            "word" => None
        }
    }
}
