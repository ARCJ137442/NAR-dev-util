//! 定义一个用于「广度优先遍历」的迭代器
//! * 📝【2024-03-02 11:57:46】对此中「内部元素」的定义：到底是「不用索引」
//!   * ❌若采用「获得元素所有权」，就会遇到「所有权问题」：
//!     * "use of moved value: `next`"无法「移动」元素，只能「拷贝」
//!   * 💭若无需修改元素，则只需持有其不可变引用

use std::collections::VecDeque;

/// 通过「搜索扩展」从「被扩展元素」返回的「邻接元素」类型
///
/// ! 📌【2024-03-02 13:19:15】不采用抽象的「迭代器」类型：简化后续实现逻辑
/// * 💫一些complicate的问题：
///   * 迭代出「值」还是迭代出「引用」？
///   * 动态迭代器用`impl T`还是包装到一个`Box<dyn T>`
///   * 函数闭包要如何兼顾性能？是否一定要压榨到极致？
type Expanded<T> = Vec<T>;
// type Expanded<T> = Box<dyn Iterator<Item = T>>;

/// 将一个元素的不可变引用进行扩展，得到其它元素的不可变引用
/// * 🚩只读获取「被扩展元素」（不可变引用），返回「扩展到的元素」（迭代器）
///
/// ! 📝【2024-03-02 11:48:07】此处不使用迭代器`impl Iterator<Item = &T>`，因为其内存大小不确定
/// ! 🚩【2024-03-02 11:48:07】此处现通过「装箱」返回更通用的迭代器（结合`into_iter`使用）
// type ExpandF<T> = dyn ;

/// BFT迭代器
///
/// ! 📝无法使用`derive`：存储函数/闭包的[`Box`]无法展示、拷贝、取默认值
// #[derive(Debug, Clone, Default)]
pub struct BFTIterator<T: PartialEq + Copy, F: Fn(T) -> Expanded<T>> {
    /// 待访问的点
    ///
    /// ! 无需「起始点」：待访问点的初值即为「起始点」
    to_visit: VecDeque<T>,
    /// 已访问的点
    ///
    /// ! 只存储引用，避免和`to_visit`冲突
    visited: Vec<T>,
    /// 扩展函数
    /// * 🚩目前通过装箱存储动态对象（闭包/函数指针）
    /// * 类型参见[`ExpandF`]
    expand_f: F,
}

impl<T: PartialEq + Copy, F: Fn(T) -> Expanded<T>> BFTIterator<T, F> {
    pub fn new(start: impl Iterator<Item = T>, expand_f: F) -> Self {
        BFTIterator {
            to_visit: start.collect(),
            visited: Vec::new(),
            // * ✅【2024-03-02 14:16:26】现在通过泛型参数`F`，装箱不装箱都可以传入了
            expand_f,
        }
    }
}

impl<T: PartialEq + Copy, F: Fn(T) -> Expanded<T>> Iterator for BFTIterator<T, F> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // * 尝试获取（无⇒直接传播）
        let next = self.to_visit.pop_front()?;

        // * 标记当前为「已访问」
        self.visited.push(next);

        // * 开始遍历并扩展「待访问」队列 | 📝Rust中强制要求「作为`Fn`对象的属性」加上花括号才调用
        for to_append in (self.expand_f)(next) {
            // * 若非已访问且不在「待访问队列」中，则加入待访问队列
            if !(self.visited.contains(&to_append) || self.to_visit.contains(&to_append)) {
                self.to_visit.push_back(to_append);
            }
        }

        // * 返回
        Some(next)
    }
}

///  单元测试
#[cfg(test)]
mod tests {
    use crate::show;

    use super::*;

    #[test]
    fn test() {
        // * 测试1：应该是顺序递减的
        let iter = BFTIterator::new(
            // * 📝使用`into_iter`遍历「具有所有权的元素」
            [10].into_iter(),
            Box::new(|u| {
                match u {
                    // 0⇒结束
                    0 => vec![],
                    // 其它⇒减1
                    n => vec![n - 1],
                }
                // .into_iter()
            }),
        );
        assert_eq!(
            show!(iter.collect::<Vec<usize>>()),
            [10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        );
        // * 测试2：同样只有「顺序递减」
        let iter = BFTIterator::new(
            [10, 9, 8].into_iter(),
            Box::new(|u| {
                match u {
                    0 => vec![],
                    1 => vec![0],
                    n => vec![n - 1, n - 2],
                }
                // .into_iter()
            }),
        );
        assert_eq!(
            show!(iter.collect::<Vec<usize>>()),
            [10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        );
        // * 测试3：肯定会遍历完
        let iter = BFTIterator::new(
            [10, 9, 8].into_iter(),
            Box::new(|u| {
                // 著名的「乘三加一除以二」逻辑
                match u {
                    1 => vec![],
                    n if n & 1 == 0 => vec![u >> 1],
                    n => vec![3 * n + 1],
                }
                // .into_iter()
            }),
        );
        show!(iter.collect::<Vec<usize>>());
    }
}
