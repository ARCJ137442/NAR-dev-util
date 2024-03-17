//! 存储一些实用的数组辅助结构
//! * 📌自动有序向量

use crate::search_by;

/// 自动有序向量
/// * 🎯始终保持元素具有一定顺序
///   * 有「要求Ord版本」与「自定义标准版本」
#[derive(Debug, Clone, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct AutoOrderedVec<T> {
    /// 数组元素
    data: Vec<T>,
}

/// 部分复现[`Vec`]的方法
impl<T> AutoOrderedVec<T> {
    /// 构造函数
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// 以一定容量构造
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    /// 获取指定位置的元素
    /// * 📌不改变元素的位置
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    /// 获取指定位置的元素（可变）
    /// * 📌不改变元素的位置
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }
}

/// 实现独有方法
impl<T: Ord> AutoOrderedVec<T> {
    /// 搜索一个元素
    /// * 📌使用包自身启用的查找算法
    /// * 🚩
    pub fn search(&self, item: &T) -> Result<usize, usize> {
        // 此处可以借助`T`的`cmp`方法进行比较
        search_by(&self.data, item, T::cmp)
    }

    /// 插入一个元素
    /// * 🚩总是会进行插入，然后返回已插入之元素的位置
    pub fn insert(&mut self, item: T) -> usize {
        // 先搜索获取「应该插入的索引」
        let index = match self.search(&item) {
            Ok(i) => i,
            Err(i) => i,
        };
        // 然后直接插入
        self.data.insert(index, item);
        index
    }

    /// 插入一个元素（保证唯一）
    /// * 🚩只在「查找不存在」时插入元素，所以返回可选值
    pub fn insert_unique(&mut self, item: T) -> Option<usize> {
        match self.search(&item) {
            // 仅在没有时插入
            Err(index) => {
                self.data.insert(index, item);
                Some(index)
            }
            // 有的时候不插入
            Ok(..) => None,
        }
    }
}

/// 单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_ordered_vec() {
        let mut vec = AutoOrderedVec::new();
        assert_eq!(vec.get(0), None);
        assert_eq!(vec.get(1), None);

        vec.insert(2);
        assert_eq!(vec.get(0), Some(&2));
        assert_eq!(vec.get(1), None);

        vec.insert(1);
        assert_eq!(vec.get(0), Some(&1));
        assert_eq!(vec.get(1), Some(&2));
    }
}
