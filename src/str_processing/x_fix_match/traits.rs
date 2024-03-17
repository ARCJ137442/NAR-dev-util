//! 前后缀匹配的抽象特征
//! * 🎯用于后续可能「同时实现『前缀匹配』与『后缀匹配』两者」的情况

use crate::str_processing::char_slices::*;
use std::cmp::Ordering;

/// 定义「前缀」
/// * 🎯统一表达[`String`]类型
pub(super) type Prefix = String;
/// 定义「前缀引用」
/// * 🎯统一表达[`str`]类型
/// * 🎯为「在基于`&str`的类型中实现」作铺垫
///   * 「通过条目获取前缀」可以只返回此类型，从而无需构造[`String`]
pub(super) type PrefixStr = str;

/// 定义「后缀」
/// * 🎯统一表达[`String`]类型
/// 定义「后缀引用」
/// * 🎯统一表达[`str`]类型
/// * 🎯为「在基于`&str`的类型中实现」作铺垫
///   * 「通过条目获取后缀」可以只返回此类型，从而无需构造[`String`]
pub(super) type Suffix = String;
pub(super) type SuffixStr = str;

/// 前缀匹配（抽象特征）
/// * 🎯用于存储前缀，封装如下两个逻辑
///   * 前缀匹配→返回被匹配项：用于匹配如「原子词项前缀」的一次性匹配
///   * 前缀匹配→返回前缀、后缀：用于匹配如「不同自定义括弧」的「配对性匹配」
///     * 🎯可以省去另一个字典映射
/// * 📌其中的前缀总是[`String`]类型
///   * 并且是**不重复**的
/// * 🎯解决「短的先匹配到截断了，长的因此无法被匹配到」的问题
/// * 🚩此处不采取「条目与前缀分离」的做法
///   * 「分离式条目」可以用`条目 = (前缀, 关联内容)`模拟
pub trait PrefixMatch<PrefixTerm> {
    /// 【抽象】用于从一个「前缀条目」中获取「前缀」
    /// * 🎯用于比较、排序、匹配
    ///   * 📄在插入元素时决定位置
    ///   * 📄在前缀匹配时使用
    /// * 🚩现在返回「静态字串」而非「动态字串的引用」
    fn get_prefix_from_term(term: &PrefixTerm) -> &PrefixStr;

    // ! ❌【2024-03-17 16:37:20】「插入前缀条目」不再适合作为一个「特征方法」存在
    // * 🎯直接原因：有些「前缀匹配の实现」可能需要其它方式进行「匹配插入」
    //   * 📄「双向匹配字典」输出`&(&前缀, &后缀)`，但却需要以`(前缀, 后缀)`插入
    // * 📌根本原因：这方法并不被其它特征方法需要
    // /// 【抽象】插入一个「前缀条目」
    // /// * 🎯通用于「单纯前缀匹配」与「配对前缀匹配」
    // fn insert_prefix_term(&mut self, term: PrefixTerm);

    /// 【抽象】迭代「前缀」和「前缀条目」
    /// * 🎯用于后续匹配
    /// * ⚠️因此需要【倒序】匹配：长的字串先来，然后是短的
    ///   * 避免"&"比"&&"优先
    fn prefix_terms<'a>(&'a self) -> impl Iterator<Item = &'a PrefixTerm> + 'a
    where
        PrefixTerm: 'a;

    /// 与「指定条目的前缀」做比对
    /// * 📜默认实现：前缀 cmp 已存在 | 保持比对顺序不变
    /// * 📌不应涉及`self`
    /// * 🚩目前仍然使用「前后缀值的引用」而非「静态字串」解决
    /// * ⚠️目前所有的「reverse」都必须在此解决
    #[inline(always)]
    fn cmp_prefix(term: &PrefixTerm, prefix: &PrefixStr) -> Ordering {
        Self::get_prefix_from_term(term).cmp(prefix)
    }

    /// 开启前缀匹配
    /// * 🎯封装「前缀匹配」逻辑，通用于「单纯前缀匹配」与「配对前缀匹配」
    /// * 🚩迭代、扫描、匹配
    ///   * 1. 从一个字符串开始
    ///   * 2. 然后扫描自身所有前缀（字串从长到短）
    ///   * 3. 最后（若成功）返回匹配到的前缀所对应的「前缀条目」
    #[inline(always)]
    fn match_prefix(&self, to_match: &str) -> Option<&PrefixTerm> {
        // * ↓非迭代器版本
        // for (prefix, term) in self.prefixes_and_items() {
        //     if_return! { to_match.starts_with(prefix) => Some(term) }
        // }
        // None
        // ✅迭代器版本
        self.prefix_terms()
            .find(|&term| to_match.starts_with(Self::get_prefix_from_term(term)))
    }

    /// 开启前缀匹配（字符迭代器版本）
    /// * 🎯封装「前缀匹配」逻辑，用于「字符迭代器」兼「字符数组切片」
    ///   * ❌字符迭代器：暂时不需要 & 还是得转数组
    /// * 🚩迭代、扫描、匹配
    ///   * 1. 从一个字符串开始
    ///   * 2. 然后扫描自身所有前缀（字串从长到短）
    ///   * 3. 最后（若成功）返回匹配到的前缀所对应的「前缀条目」
    #[inline(always)]
    fn match_prefix_char_slice(&self, to_match: &[char]) -> Option<&PrefixTerm> {
        self.prefix_terms()
            .find(|&term| char_slice_has_prefix(to_match, Self::get_prefix_from_term(term)))
    }
}

/// 后缀匹配（抽象特征）
/// * 🎯用于存储后缀，封装如下两个逻辑
///   * 后缀匹配→返回被匹配项：用于匹配如「原子词项后缀」的一次性匹配
///   * 后缀匹配→返回前 缀、后 缀：用于匹配如「不同自定义括弧」的「配对性匹配」
///     * 🎯可以省去另一个字典映射
/// * 📌其中的后缀总是[`String`]类型
///   * 并且是**不重复**的
/// * 🎯解决「短的先匹配到截断了，长的因此无法被匹配到」的问题
/// * 🚩此处不采取「条目与后缀分离」的做法
///   * ℹ️「分离式条目」可以用`条目 = (关联内容, 后缀)`模拟
///     * 📌于是**不用考虑「条目」的内部结构**
///   * ⚠️亦即不涉及「关联内容」的类型
///   * ❌因此无法统一定义「后缀+关联内容→条目」的函数
///   * ❌对于「关联内容就是词缀本身」的情况：不能避免在「向函数传入参数」时「拷贝两个词缀」
///   * 🚩【2024-03-17 13:12:31】将「管理『条目』与『关联内容的接口」统一放在「具体类型实现」中
///     * ❗但还是需要定义「条目→后缀」的接口
pub trait SuffixMatch<SuffixTerm> {
    /// 【抽象】用于从一个「后缀条目」中获取「后缀」
    /// * 🎯用于比较、排序、匹配
    ///   * 📄在插入元素时决定位置
    ///   * 📄在后缀匹配时使用
    /// * 🚩现在返回「静态字串」而非「动态字串的引用」
    fn get_suffix_from_term(term: &SuffixTerm) -> &SuffixStr;

    // ! ❌【2024-03-17 16:37:20】「插入前缀条目」不再适合作为一个「特征方法」存在
    // * 🎯直接原因：有些「前缀匹配の实现」可能需要其它方式进行「匹配插入」
    //   * 📄「双向匹配字典」输出`&(&前缀, &后缀)`，但却需要以`(前缀, 后缀)`插入
    // * 📌根本原因：这方法并不被其它特征方法需要
    // /// 【抽象】插入一个「后缀条目」
    // /// * 🎯通用于「单纯后缀匹配」与「配对后缀匹配」
    // fn insert_suffix_term(&mut self, term: SuffixTerm);

    /// 【抽象】迭代「后缀」和「后缀条目」
    /// * 🎯用于后续匹配
    /// * ⚠️因此需要【倒序】匹配：长的字串先来，然后是短的
    ///   * 避免"&"比"&&"优先
    fn suffix_terms<'a>(&'a self) -> impl Iterator<Item = &'a SuffixTerm> + 'a
    where
        SuffixTerm: 'a;

    /// 与「指定条目的后缀」做比对
    /// * 📜默认实现：后缀 cmp 已存在 | 保持比对顺序不变
    /// * 📌不应涉及`self`
    /// * 🚩目前仍然使用「前后缀值的引用」而非「静态字串」解决
    /// * ⚠️目前所有的「reverse」都必须在此解决
    #[inline(always)]
    fn cmp_suffix(term: &SuffixTerm, suffix: &SuffixStr) -> Ordering {
        Self::get_suffix_from_term(term).cmp(suffix)
    }

    /// 开启后缀匹配
    /// * 🎯封装「后缀匹配」逻辑，通用于「单纯后缀匹配」与「配对后缀匹配」
    /// * 🚩迭代、扫描、匹配
    ///   * 1. 从一个字符串开始
    ///   * 2. 然后扫描自身所有后缀（字串从长到短）
    ///   * 3. 最后（若成功）返回匹配到的后缀所对应的「后缀条目」
    #[inline(always)]
    fn match_suffix(&self, to_match: &str) -> Option<&SuffixTerm> {
        // * ↓非迭代器版本
        // for (suffix, term) in self.suffixes_and_items() {
        //     if_return! { to_match.ends_with(suffix) => Some(term) }
        // }
        // None
        // ✅迭代器版本
        self.suffix_terms()
            .find(|&term| to_match.ends_with(Self::get_suffix_from_term(term)))
    }

    /// 开启后缀匹配（字符迭代器版本）
    /// * 🎯封装「后缀匹配」逻辑，用于「字符迭代器」兼「字符数组切片」
    ///   * ❌字符迭代器：暂时不需要 & 还是得转数组
    /// * 🚩迭代、扫描、匹配
    ///   * 1. 从一个字符串开始
    ///   * 2. 然后扫描自身所有后缀（字串从长到短）
    ///   * 3. 最后（若成功）返回匹配到的后缀所对应的「后缀条目」
    #[inline(always)]
    fn match_suffix_char_slice(&self, to_match: &[char]) -> Option<&SuffixTerm> {
        self.suffix_terms()
            .find(|&term| char_slice_has_suffix(to_match, Self::get_suffix_from_term(term)))
    }
}
