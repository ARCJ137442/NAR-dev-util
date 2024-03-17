//! 用于定义「前后缀都不重复」的「双向配对字典」
//! * 🎯用于「括弧匹配」情形
//!   * ℹ️此时括弧一般都两两不重复
//! * ✨可由前缀搜后缀，亦可后缀搜前缀

use super::traits::*;
use crate::{search_by, PrefixMatchDictPair, SuffixMatchDictPair};

/// 「双向配对条目」
/// * 🎯实际就是`(前缀, 后缀)`的简写
type BiFixTerm<P = Prefix, S = Suffix> = (P, S);

// /// 「双向配对引用条目」
// /// * 🎯实际就是`(&前缀, &后缀)`的简写
// type BiFixRefTerm<'a, P = Prefix, S = Suffix> = BiFixTerm<&'a P, &'a S>;
// ! ❌【2024-03-17 17:15:59】随着「弃用『条目引用序列』」

/// 双向配对字典
/// * 🎯用于（通常左右括弧不重复的）括弧匹配场景
///   * ✨既能前缀匹配（返回后缀），也能后缀匹配（返回前缀）
/// * 📌虽「前缀⇔后缀」为双射，但「前缀の排列」不一定与「后缀の排列」相同
///   * 📄case: `("a", "")` & `("", "a")` ←这两者在「前缀匹配」与「后缀匹配」时有不同的顺序
///   * 💡但可以存储与其相关的引用
///
/// ---
///
/// * 📜【2024-03-17 17:06:22】使用「前缀配对字典 + 后缀序索引序列」的「捆绑」方式
///   * 📌通过「前缀配对字典」实现「前后缀存储」与「前缀顺序信息」
///   * 📌通过「后缀序索引序列」实现「后缀顺序信息」
///   * 💭即便有些「不对称」也无所谓（也可以「后缀配对字典 + 前缀引用序列」）
///   * 📍引用结构
///     * `条目 = (前缀, 后缀)` in 前缀匹配字典 | 数据所有权、前缀顺序信息
///     * `后缀序索引序列 = Vec<索引 in 前缀匹配字典の前缀集>` | 按（索引到的）后缀排序，存储后缀顺序信息
///       * ✅同时在「按后缀顺序迭代条目」时能直接返回
///   * 📍空间复杂度仅为`n(2S + usize)`
///     * n: 条目数量
///     * S: 前缀/后缀大小
///   * ❗非必要莫尝试「在容器之外持久化存储对容器的引用」
///
/// ---
///
/// * 📜【2024-03-17 17:06:22】使用三个数组「纯原始实现」，不再「直接封装」「东拼西凑」
///   * 【2024-03-17 17:19:26】弃用：直接「前缀配对字典 + 后缀引用序列」亦可支持
///   * 💥不再内含「前缀配对字典」与「后缀配对字典」
///   * 📌通过「引用序列的顺序」实现「预排序」
///   * 📍引用结构
///     * `条目 = (前缀, 后缀)` | 存储数据所有权、前后缀关系
///     * `前缀引用序列 = Vec<&前缀>` | 存储前缀顺序信息
///     * `后缀引用序列 = Vec<&后缀>` | 存储后缀顺序信息
///   * 📍空间复杂度仅为`2n(S + &S)`
///     * n: 条目数量
///     * S: 前缀/后缀大小
/// * ❌【2024-03-17 16:18:35】现在使用「条目引用序列 + 前后缀配对字典」的方法存储
///   * 📌条目只存储引用，分别指向前后缀配对字典
///   * 🎯顺应「配对字典」的「前后缀必须有所有权」要求
///   * 📍引用结构：`条目 = (&前缀条目(前缀, &条目), &后缀条目(&条目, 后缀))`
///     * ❗包含循环引用（条目包含「&条目」），无法正确构造
///   * 📝事后笔记：safe环境下不要尝试构造「包含『循环引用』的结构」
/// * ❌【2024-03-17 11:13:12】使用「条目序列 + 前后缀引用配对字典」的方法存储
///   * 📌「前缀配对字典」与「后缀配对词典」都关联「条目序列」中的项
///   * 🎯在「迭代前缀/迭代后缀」时，需要【一次性返回整个条目的引用】而避免「东拼西凑」
///   * 📝对于「返回复杂数据之引用」的迭代器实现，最好的方法只能是「迭代出的数据本身就在内部存在」
///     * ❗否则就要走`.collect::<Vec<_>>().into_iter()`的下策（内存开销）
#[derive(Debug, Clone, Default)]
pub struct BiFixMatchDictPair {
    /// 内部封装的「前缀配对字典」
    /// * 📌存储「后缀」
    /// * 🚩后缀唯一性在插入时判定
    prefix_dict: PrefixMatchDictPair<Suffix>,

    /// 后缀序索引序列
    /// * 用于存储按后缀的排列顺序（后缀字母序反向）
    /// * 🚩现在存储索引，而非索引指向的引用
    ///   * 至少不用直接存储引用（可能导致生命周期问题）
    suffix_ordered_refs: Vec<usize>,
}

impl BiFixMatchDictPair {
    /// 构造函数
    /// * ⚠️实际上不推荐从所谓「迭代器」直接创建数组：**难以预先排序**
    /// * 🚩现在采用「先新建空值，然后逐个添加」来实现
    ///   * 📌复杂度：∑ 1 log 1 ~ n log n
    /// * 📌格式：`条目=(其它元素, 后缀)`
    pub fn new(
        bi_fixes: impl IntoIterator<Item = BiFixTerm<impl Into<Prefix>, impl Into<Suffix>>>,
    ) -> Self {
        // ? 后续可以抽象提取`insert_suffix_terms`乃至`insert_terms`？
        // 使用`default`创建一个空值
        let mut dict = Self::default();
        // 逐个添加
        for term in bi_fixes.into_iter() {
            let term = Self::new_term(term.0.into(), term.1.into());
            dict.insert(term);
        }
        dict.insert((String::new(), String::new()));
        // 返回
        dict
    }

    /// 从「前缀」与「后缀」构造「条目」
    #[inline(always)]
    pub fn new_term(prefix: Prefix, suffix: Suffix) -> BiFixTerm {
        (prefix, suffix)
    }

    /// 从「后缀序索引」获取条目
    /// * 🚩实际上转发到「前缀配对字典」
    /// * ⚠️**调用者注意：需要检查索引是否在界内**
    pub(super) fn get_term_by_index(&self, index: usize) -> Option<&BiFixTerm> {
        self.prefix_dict.prefixes.get(index)
    }

    /// 统一的「插入」方法
    /// * 🎯前后缀对称
    /// * 🚩要确保「前缀」「后缀」各自唯一
    /// * 🚩返回「是否成功插入」
    pub fn insert(&mut self, term: BiFixTerm) -> bool {
        // 先确保「后缀唯一」
        let search_result = self.search_suffix(SuffixMatchDictPair::get_suffix_from_term(&term));
        // 新后缀⇒插入
        if let Err(i_insert) = search_result {
            // 先尝试将元素插入「前缀配对字典」
            // * 🎯返回并继续持有条目引用
            if let Some(i_term) = self.prefix_dict.insert(term) {
                // * ⚠️「前缀配对字典」中的「条目索引」会随着插入而改变
                // * 需要在每次插入前更新「后缀序索引」中的「前缀配对字典条目索引」
                //   * 🚩更新规则：先前大于等于自己的⇒自增1
                self.suffix_ordered_refs
                    // 获取可变引用
                    .iter_mut()
                    // 先前大于「要插入的地方」的
                    .filter(|i_prefix_index| **i_prefix_index >= i_term)
                    // 自增1
                    .for_each(|i_prefix_index| *i_prefix_index += 1);
                // ! 不要插入引用，插入索引
                self.suffix_ordered_refs.insert(i_insert, i_term);
                return true;
            }
        }
        false
    }

    /// 搜索前缀
    /// * 🚩直接转发到「前缀字典」
    pub fn search_prefix(&self, prefix: &PrefixStr) -> Result<usize, usize> {
        self.prefix_dict.search(prefix)
    }

    /// 搜索后缀
    /// * 📌直接使用内置的「搜索算法」查找
    /// * 🚩按后缀搜索
    pub fn search_suffix(&self, suffix: &SuffixStr) -> Result<usize, usize> {
        search_by(&self.suffix_ordered_refs, &suffix, |suffix, term_index| {
            // ! 此时因为是在「后缀」自身中搜索，故一定确保索引正确
            let term_ref = self.get_term_by_index(*term_index).unwrap();
            SuffixMatchDictPair::cmp_suffix(term_ref, suffix)
        })
    }
}

#[macro_export]
macro_rules! bi_fix_match_dict_pair {
    // 转换其中的值 | 静态字串⇒动态字串 自动`into`
    (@value $v:literal) => {
        $v.into()
    };
    // 转换其中的值 | 表达式⇒直接加入
    (@value $v:expr) => {
        $v
    };
    // 统一的表 | 自面量也是一种表达式
    [$($bi_fix:expr => $item:expr $(,)?)*] => {{
        let mut d = BiFixMatchDictPair::default();
        $(
            d.insert((
                bi_fix_match_dict_pair!(@value $bi_fix),
                bi_fix_match_dict_pair!(@value $item),
            ));
        )*
        d
    }};
}

/// 实现「前缀匹配」
impl PrefixMatch<BiFixTerm> for BiFixMatchDictPair {
    // 重定向到「前缀配对字典」的方法
    fn get_prefix_from_term(term: &BiFixTerm) -> &PrefixStr {
        PrefixMatchDictPair::get_prefix_from_term(term)
    }

    // 重定向到内部的「前缀配对字典」
    fn prefix_terms<'a>(&'a self) -> impl Iterator<Item = &'a BiFixTerm> + 'a
    where
        BiFixTerm: 'a,
    {
        self.prefix_dict.prefix_terms()
    }
}

/// 实现「后缀匹配」
impl SuffixMatch<BiFixTerm> for BiFixMatchDictPair {
    // 重定向到内部的「后缀配对字典」的方法
    fn get_suffix_from_term(term: &BiFixTerm) -> &SuffixStr {
        SuffixMatchDictPair::get_suffix_from_term(term)
    }

    fn suffix_terms<'a>(&'a self) -> impl Iterator<Item = &BiFixTerm> + 'a
    where
        BiFixTerm: 'a,
    {
        // * 直接在「后缀序索引序列」
        // ! ⚠️此处必须确保索引有效
        self.suffix_ordered_refs
            .iter()
            .map(|&index| self.get_term_by_index(index).unwrap())
    }
}

/// 单元测试/双向匹配
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{show, test_match_prefix, test_match_suffix};

    /// 测试/边缘
    #[test]
    fn test_edge() {
        // 构造测试用例
        let d: BiFixMatchDictPair = bi_fix_match_dict_pair!(
            "" => "" // 空值fallback
            "a" => "b"
            "aa" => "bb"
            "aaa" => "bbb"
        );
        show!(&d);
        // 开始前缀匹配
        test_match_prefix! {
            d;
            // 完全匹配
            "a" => Some("b")
            "aa" => Some("bb")
            "aaa" => Some("bbb")
            // 范围内情况
            "a_" => Some("b")
            "aa_" => Some("bb")
            "aaa_" => Some("bbb")
            // 空值fallback
            "" => Some("")
            "x" => Some("")
        }
        // 开始后缀匹配
        test_match_suffix! {
            d;
            // 完全匹配
            "b" => Some("a")
            "bb" => Some("aa")
            "bbb" => Some("aaa")
            // 范围内情况
            "_b" => Some("a")
            "_bb" => Some("aa")
            "_bbb" => Some("aaa")
            // 空值fallback
            "" => Some("")
            "x" => Some("")
        }
    }

    /// 测试/实战
    #[test]
    fn test_bi_fix_match_pairs() {
        // 构造测试用例 | 双向括弧匹配
        let d: BiFixMatchDictPair = bi_fix_match_dict_pair!(
            "(" => ")"
            "[" => "]"
            "{" => "}"
            "<" => ">"
        );
        show!(&d);
        // 测试前缀匹配 | 前缀⇒后缀
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
        // 测试后缀匹配 | 后缀⇒前缀
        test_match_suffix! {
            d;
            // 范围内情况
            r"(A, B, C)" => Some("(")
            r"[A, B, C]" => Some("[")
            r"{A, B, C}" => Some("{")
            r"<A, B, C>" => Some("<")
            // 无效情况
            "word" => None
        }
    }
}
