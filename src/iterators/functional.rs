/// 函数式迭代器
/// * 🎯最初用于「基于**闭包/函数指针**灵活定义迭代器」
/// * 🚩直接定义一个[`FnIterator::next`]，并直接在[`Iterator::next`]中执行
///   * 📌调用时的「可变元素」基本依赖闭包捕获的外部变量
/// * ❌无法用于从「缓冲区迭代器」生成「头迭代器」：无法返回指向可变闭包[`FnMut`]的内部变量的引用
///   * 📌弃用原因：闭包的所有权问题
/// * ❗标准库中已经有集成了：参见[`std::iter::from_fn`]
///   * 📝一些细节实现差异
///     * 📌标准库直接使用单元组struct，而本struct使用普通结构体
///     * 📌标准库把「函数类型限制」从「定义时」留到了「实现时」
///   * 🚩【2024-03-10 11:15:11】代码计划封存，以和标准库作对比
pub struct FnIterator<F, T>
where
    F: FnMut() -> Option<T>,
{
    f: F,
}

impl<F, T> FnIterator<F, T>
where
    F: FnMut() -> Option<T>,
{
    /// 构造函数：直接基于 函数指针/闭包 创建迭代器
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

/// 实现标准迭代器接口
impl<F, T> Iterator for FnIterator<F, T>
where
    F: FnMut() -> Option<T>,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        (self.f)()
    }
}

/// 函数式迭代器
#[test]
fn test_functional_iter() {
    // 构造一个「不断迭代'a'」的迭代器
    let item = 'a';
    let mut iter = FnIterator::new(|| Some(item));
    const N: usize = 100000;
    for _ in 0..N {
        // 肯定迭代出元素，并且恒等于'a'
        assert_eq!(iter.next().unwrap(), item);
    }

    // 构造一个`i32`的空迭代器
    let iter = FnIterator::new(|| None::<i32>);
    assert_eq!(iter.count(), 0); // 不会有计数

    // 构造一个斐波那契迭代器
    let mut a_n1: usize = 0;
    let mut a_n2: usize = 0;
    let mut a_n3: usize = 1;
    let mut iter = FnIterator::new(|| {
        // 计算新数据
        a_n1 = a_n2;
        a_n2 = a_n3;
        a_n3 = a_n1 + a_n2;
        // 返回数据
        Some(a_n2)
    });
    assert_eq!(iter.nth(10 - 1).unwrap(), 55); // `10-1`才是「第10个」
}
