//! 与「后缀匹配」有关的工具结构与算法
//! * 🎯最初用于字符串parser
//! * 🎯存储专用的「后缀匹配」实现
//!   * 📌在「标点」「真值」用于「后缀匹配」
//!   * 📌在「时间戳」用于「空前缀匹配」
//! * 📜此文件是直接从「前缀匹配」拿过来的
//!   * 💭虽说要同时维护，但情况较少

use super::traits::*;

/// 「后缀条目」
/// * 🎯统一表达`(关联内容, 后缀)`的二元组
type SuffixTerm<T, XFix = Suffix> = (T, XFix);

/// 后缀配对字典
/// * 🚩具体逻辑：
///   * 【倒序】维护一个有一定顺序、不重复的[`String`]数组
///   * 使用「二元组」直接在数组内建立「字符串⇒其它元素」的关联
#[derive(Debug, Clone)]
pub struct SuffixMatchDictPair<T> {
    /// * 🎯此处设计成`(关联内容, 后缀)`元组顺序，纯粹为了代码上`("<", ">")`对称
    pub(super) suffixes: Vec<SuffixTerm<T>>,
}

/// 实现「默认构造函数」
/// * 🚩通过「初始化空数组」完成
impl<T> Default for SuffixMatchDictPair<T> {
    fn default() -> Self {
        Self {
            suffixes: Vec::new(),
        }
    }
}

/// 通过宏快捷构造「后缀配对字典」
/// * 📌格式：「前 => 后」
#[macro_export]
macro_rules! suffix_match_dict_pair {
    // 转换其中的值 | 静态字串⇒动态字串 自动`into`
    (@value $v:literal) => {
        $v.into()
    };
    // 转换其中的值 | 表达式⇒直接加入
    (@value $v:expr) => {
        $v
    };
    // 统一的表 | 自面量也是一种表达式
    [$($suffix:expr => $item:expr $(,)?)*] => {{
        let mut d = $crate::SuffixMatchDictPair::default();
        $(
            d.insert((
                suffix_match_dict_pair!(@value $suffix),
                suffix_match_dict_pair!(@value $item),
            ));
        )*
        d
    }};
}

/// 实现专用方法
impl<T> SuffixMatchDictPair<T> {
    /// 构造函数
    /// * ⚠️实际上不推荐从所谓「迭代器」直接创建数组：**难以预先排序**
    /// * 🚩现在采用「先新建空值，然后逐个添加」来实现
    ///   * 📌复杂度：∑ 1 log 1 ~ n log n
    /// * 📌格式：`条目=(其它元素, 后缀)`
    pub fn new(suffixes: impl IntoIterator<Item = SuffixTerm<T, impl Into<Suffix>>>) -> Self {
        // ! ❌【2024-03-17 16:42:39】不再尝试提取构造函数
        // 使用`default`创建一个空值
        let mut dict = Self::default();
        // 逐个添加
        for term_to_into in suffixes.into_iter() {
            // 针对`Into`做一个「转换再插入」
            // ! 仍然无法使用`get_associated_from_term`：这时候的`term_to_into`还不是`term`
            dict.insert(Self::new_suffix_term(term_to_into.1.into(), term_to_into.0));
        }
        // 返回
        dict
    }

    /// 后缀条目→后缀（引用）
    /// * 📌是`&String`而非`&str`，真正引用到后缀实例本身
    /// * ⚠️【2024-03-17 16:04:52】目前暂无法提取至特征
    ///   * 📌原因：对「词缀匹配字典」无法容忍「(自身, 自身)」参数
    #[inline(always)]
    pub fn suffix_ref_of(term: &SuffixTerm<T>) -> &Suffix {
        &term.1
    }

    /// 用于从一个「后缀条目」中获取「关联内容」
    /// * ⚠️【2024-03-17 13:05:43】目前暂无法提取至特征
    ///   * 📌原因：对「词缀匹配字典」无法容忍「(自身, 自身)」参数
    #[inline(always)]
    pub fn get_associated_from_term(term: &SuffixTerm<T>) -> &T {
        &term.0
    }

    /// 从「后缀」与「关联内容」组装「后缀条目」
    /// * ⚠️【2024-03-17 13:05:43】目前暂无法提取至特征
    ///   * 📌原因：对「词缀匹配字典」无法容忍「(自身, 自身)」参数
    #[inline(always)]
    pub fn new_suffix_term(suffix: Suffix, associated: T) -> SuffixTerm<T> {
        (associated, suffix)
    }

    /// 判断「是否已有一个后缀」
    /// * 📌直接使用自身的「搜索」功能
    /// * ⚠️因涉及【涉及内部字段】的[`Self::search`]方法，无法提取至特征
    #[inline(always)]
    pub fn has(&self, suffix: &Suffix) -> bool {
        // * 🚩查找「ok」证明「能找到」
        self.search(suffix).is_ok()
    }

    /// 插入一个条目
    /// * 🚩调用经分派的「查找」方法
    /// * 🚩返回「插入后元素位置的索引」
    ///   * 🎯相比「是否插入」提供了「插入后的元素」的信息
    ///   * 📝**若返回「插入后元素的引用」，则会导致字段连带自身被持续借用**
    ///     * ❗因此导致「借了不还」的生命周期问题
    ///     * ✨此时还不如返回索引，让调用者根据索引调用
    pub fn insert(&mut self, term: SuffixTerm<T>) -> Option<usize> {
        // 只有在「查找失败」时进行插入
        if let Err(index) = self.search(Self::get_suffix_from_term(&term)) {
            // 直接调用数组的「插入」方法
            self.suffixes.insert(index, term);
            return Some(index);
        }
        // 插入失败⇒失败
        None
    }

    /// （前后缀无关）以特殊顺序迭代「词缀」
    /// * 🎯统一「前后缀匹配」的迭代逻辑
    /// * 🚩总是按照「字典顺序」倒序遍历：**长度从长到短**
    #[inline(always)]
    pub fn iter_terms<'a>(&'a self) -> impl Iterator<Item = &SuffixTerm<T>> + 'a
    where
        String: 'a,
    {
        // ! 【2024-03-17 12:34:00】此处因为「原先就以倒序存储」所以直接顺序遍历
        self.suffixes.iter()
    }

    /// 搜索 | 使用二分查找
    /// * 🎯构造可方便替换的「查找」逻辑
    /// * 🚩找到⇒位置，没找到⇒应该插入的位置
    ///   * 📌这个「应该插入的位置」需要**让插入后「从大到小排列」**
    ///   * 📌亦即【渐进式构建】「后缀从长到短的数组」
    /// * 📌【2024-03-17 12:13:14】实际上对「待插入的条目」只需要其后缀信息
    ///   * 故直接根据后缀寻找
    /// * ⚠️因涉及「内部数组」所以【无法提取至通用特征】
    #[inline(always)]
    pub fn search(&self, suffix: &SuffixStr) -> Result<usize, usize> {
        super::search_by(&self.suffixes, &suffix, |suffix, existed| {
            // ! ⚠️不要在此添加`.reverse()`：限定在`cmp之内解决
            // * 📌保证插入后「比自己大的 > 自己 > 已存在」
            Self::cmp_suffix(existed, suffix) // ←由二分查找的「从小到大」逆转为「从大到小」
        })
    }
}

/// 实现「后缀匹配」逻辑
impl<T> SuffixMatch<SuffixTerm<T>> for SuffixMatchDictPair<T> {
    // 下面的方法直接进行「特化重定向」处理 //
    fn get_suffix_from_term(term: &SuffixTerm<T>) -> &SuffixStr {
        // 返回的「后缀引用」【自动转换】为静态引用 | &String -> &str
        Self::suffix_ref_of(term)
    }
    fn suffix_terms<'a>(&'a self) -> impl Iterator<Item = &'a SuffixTerm<T>> + 'a
    where
        SuffixTerm<T>: 'a,
    {
        self.iter_terms()
    }
}

/// 单元测试/后缀匹配
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{show, test_match_suffix};

    /// 测试/边缘
    #[test]
    fn test_edge() {
        //构造测试用例
        let d: SuffixMatchDictPair<String> = suffix_match_dict_pair!(
            "0" => ""  // 空值fallback
            "1" => "a"
            "2" => "aa"
            "3" => "aaa"
        );
        show!(&d);
        // 开始匹配
        test_match_suffix! {
            d;
            // 完全匹配
            "a" => Some("1")
            "aa" => Some("2")
            "aaa" => Some("3")
            // 范围内情况
            "_a" => Some("1")
            "_aa" => Some("2")
            "_aaa" => Some("3")
            // 空值fallback
            "" => Some("0")
            "b" => Some("0")
        }
    }

    /// 测试/实战
    #[test]
    fn test_suffix_match_pairs() {
        // 测试「括弧匹配」
        let d: SuffixMatchDictPair<String> = suffix_match_dict_pair!(
            "(" => ")"
            "[" => "]"
            "{" => "}"
            "<" => ">"
        );
        show!(&d);
        // 测试后缀匹配
        test_match_suffix! {
            d;
            // 范围内情况 | ❗后缀⇒前缀
            r"(A, B, C)" => Some("(")
            r"[A, B, C]" => Some("[")
            r"{A, B, C}" => Some("{")
            r"<A, B, C>" => Some("<")
            // 无效情况
            "word" => None
        }

        // 测试「真值」「时间戳」匹配
        let d: SuffixMatchDictPair<String> = suffix_match_dict_pair!(
            // 真值 //
            "%" => "%"
            r"\langle{}" => r"\rangle{}"
            "真" => "值"
            // 时间戳 //
            // ASCII
            "" => r":\:" // 过去
            "" => r":|:" // 现在
            "" => r":/:" // 将来
            ":!" => r":" // 固定
            // LaTeX
            "" => r"\backslash\!\!\!\Rightarrow{}" // 过去
            "" => r"|\!\!\!\Rightarrow{}" // 现在
            "" => r"/\!\!\!\Rightarrow{}" // 将来
            "t=" => "", // 固定 // ! 空值fallback
            // 漢文
            "" => "过去" // 过去
            "" => "现在" // 现在
            "" => "将来" // 将来
            // "发生在" => "", // ! ←此处重复了
        );
        show!(&d);
        // 测试后缀匹配
        test_match_suffix! {
            d;
            // 范围内情况 | ❗后缀⇒前缀 //
            // ASCII
            r"<A --> B>. :\:" => Some("") // ! 空前缀策略
            r"<A --> B>. :|:" => Some("") // ! 空前缀策略
            r"<A --> B>. :/:" => Some("") // ! 空前缀策略
            r"<A --> B>. :!-137:" => Some(r":!")
            // LaTeX
            r"\left<A \rightarrow{} B\right>. \backslash\!\!\!\Rightarrow{}" => Some("")
            r"\left<A \rightarrow{} B\right>. |\!\!\!\Rightarrow{}" => Some("")
            r"\left<A \rightarrow{} B\right>. /\!\!\!\Rightarrow{}" => Some("")
            r"\left<A \rightarrow{} B\right>." => Some("t=") // ! fallback
            // 漢文
            "「A是B」。过去" => Some("")
            "「A是B」。现在" => Some("")
            "「A是B」。将来" => Some("")
        }
    }
}
