use std::collections::VecDeque;

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

// ! ❌【2024-03-04 20:58:35】实践：因为「打包后需要从中借用值」的借用问题，再次弃用「独立使用『头迭代器』管理迭代过程」的想法
// /// ! ❌【2024-03-04 20:28:24】无法经由「新struct代理」为[`BufferIterator`]生成「头迭代器」（同时不获取所有权）
// /// ! 编译错误信息如下：
// /// ```plaintext
// /// error: lifetime may not live long enough
// ///   --> src\util\iterators.rs:51:21
// ///   |
// /// 42 | impl<'a, T, I> Iterator for HeadIter<'a, T, I>
// ///   |      -- lifetime `'a` defined here
// /// ...
// /// 48 |     fn next(&mut self) -> Option<Self::Item> {
// ///   |             - let's call the lifetime of this reference `'1`
// /// ...
// /// 51 |             true => Some(self.0.buffer.back().unwrap()),
// ///   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ method was supposed to return data with lifetime `'a` but it is returning data with lifetime `'1`
// /// ```
// ///
// /// ! 🚩【2024-03-04 20:32:17】不再采用「原迭代器的引用」，转向「获取所有权，然后unwrap返回」的方法
// ///  * 此处将（短暂）获取其所有权，然后可通过[`Self::unwrap`]方法解包
// // pub struct HeadIter<'a, T, I>(&'a mut BufferIterator<T, I>) // ! 不再使用引用，因此不再需要生命周期参数
// pub struct HeadIter<T, I>(BufferIterator<T, I>)
// where
//     // T: 'a, // ! 对下方「生命周期问题」无济于事
//     I: Iterator<Item = T>;

// impl<T, I> HeadIter<T, I>
// where
//     // T: 'a, // ! 对下方「生命周期问题」无济于事
//     I: Iterator<Item = T>
// {
//     /// 构造函数
//     pub fn new(iter: BufferIterator<T, I>) -> Self {
//         Self(iter)
//     }

//     /// 解包
//     /// * ⚠️消耗自身所有权
//     pub fn unwrap(self) -> BufferIterator<T, I>{
//         self.0
//     }
// }

// /// 实现标准迭代器接口
// impl<T, I> Iterator for HeadIter<T, I>
// where
//     // T: 'a, // ! 对下方「生命周期问题」无济于事
//     I: Iterator<Item = T>,
// {
//     /// ! 💭想返回引用，但这里`Item`定义`&T`需要附加生命周期；
//     /// * ❌想在结构体定义处附加生命周期标识，但却报错「未使用的生命周期类型」
//     /// * 🚩【2024-03-04 20:43:41】结论：因为生命周期问题，弃掉「返回『新增的元素』的引用」的返回类型
//     type Item = ()/* &T */;
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.0.head_next() {
//             // 有从「内部迭代器」中拿到元素⇒返回这个元素的不可变引用
//             true => Some(()/* self.0.buffer.back().unwrap() */),
//             // 没有从「内部迭代器」中拿到元素⇒返回None
//             false => None,
//         }
//     }
// }

/// 缓冲迭代器
/// * 🎯最初用于「只会从前往后解析字符串，除了『缓冲区』不会进行回溯」的字符串解析器
/// * 🚩用于**带缓冲地从某个迭代器里迭代东西**
///
/// ! ⚠️【2024-03-03 23:29:48】目前因为「需要迭代出去，同时还要缓存」要求其内元素可以被复制（实现[`Clone`]，如[`char`]）
///   * 因此，该迭代器会**自动复制**其所封装迭代器中的元素
pub struct BufferIterator<T, I>
where
    I: Iterator<Item = T>,
{
    iterator: I,
    /// 记录「已迭代未清理」的元素
    /// * 🚩使用**队列**以便在「缓冲区递进」时弹出元素
    buffer: VecDeque<T>,
    /// 记录迭代到的「头索引」（缓冲区末尾）
    /// * 可能为空：尚未开始迭代时（最开始迭代将设置在0）
    ///
    /// ! ⚠️不同于「缓冲区开头」所迭代到的索引
    head: usize,
    /// 是否开始迭代
    /// * 🎯为了在获取「头索引」时避免「获取空迭代器的头索引」
    is_began: bool,
    /// 是否迭代到了末尾
    /// * 🎯为了在获取「是否迭代完」时不修改迭代器
    is_ended: bool,
}

impl<T, I> BufferIterator<T, I>
where
    I: Iterator<Item = T>,
{
    /// 构造函数
    /// * 📌`head`初始为`0`，`is_began`初始为`false`，`is_ended`初始为`false`
    /// * 📌`buffer`初始为空
    pub fn new(iterator: I) -> Self {
        BufferIterator {
            // 载入迭代器
            iterator,
            buffer: VecDeque::new(),
            // 头索引初始化为0
            head: 0,
            // 未开始迭代，未结束迭代
            is_began: false,
            is_ended: false,
        }
    }

    /// 获取「头索引」
    /// * 📌当【缓冲区非空】时，不会随[`Self::buffer_next`]的调用而改变
    /// * ⚠️不是「缓冲区开头」所在的索引
    ///   * 后者为「缓冲区头索引」[`Self::buffer_head`]
    /// * ⚠️当自身【未开始迭代】时，「头索引」仍然为`0`
    pub fn head(&self) -> usize {
        self.head
    }

    /// 获取「缓冲区头索引」
    /// * 🚩是「缓冲区开头」所在的索引
    /// * 📌不会随[`Self::next`]的调用而改变
    /// * ⚠️当自身【未开始迭代】时，「缓冲区头索引」为`0`
    ///   * 📌「缓冲区长度」永远不会大于「头索引+1」
    ///   * 📌这也说明：**当「缓冲区头索引>头索引」时，缓冲区为空**
    pub fn buffer_head(&self) -> usize {
        (self.head + 1) - self.buffer.len()
    }

    /// 获取「头元素」（不可变引用）
    /// * 📌实际上是「缓冲区末尾元素」
    /// * 🚩缓冲区非空⇒`Some(引用)`，缓冲区为空⇒`None`
    pub fn head_item(&self) -> Option<&T> {
        self.buffer.back()
    }

    /// 获取「缓冲区头元素」（不可变引用）
    /// * 📌实际上是「缓冲区开头元素」
    /// * 🚩缓冲区非空⇒`Some(引用)`，缓冲区为空⇒`None`
    pub fn buffer_head_item(&self) -> Option<&T> {
        self.buffer.front()
    }

    /// 获取「头元素」（可变引用）
    /// * 📌实际上是「缓冲区末尾元素」
    /// * 🚩缓冲区非空⇒`Some(引用)`，缓冲区为空⇒`None`
    pub fn head_item_mut(&mut self) -> Option<&mut T> {
        self.buffer.back_mut()
    }

    /// 获取「缓冲区头元素」（可变引用）
    pub fn buffer_head_item_mut(&mut self) -> Option<&mut T> {
        self.buffer.front_mut()
    }

    /// 获取「是否已开始」
    pub fn is_began(&self) -> bool {
        self.is_began
    }

    /// 获取「是否迭代完」
    pub fn is_ended(&self) -> bool {
        self.is_ended
    }

    /// 获取「缓冲区长度」
    pub fn len_buffer(&self) -> usize {
        self.buffer.len()
    }

    /// 判断「缓冲区是否为空」
    pub fn is_buffer_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// 头迭代
    /// * 🚩尝试从「内部迭代器」中迭代出一个值，然后尝试返回这个值的**不可变引用**
    ///   * ⚠️可能会迭代结束，此时返回[`None`]
    ///   * ❌无法返回可变引用："cannot mutate immutable variable `item`"
    ///
    /// * ℹ️要使用「头迭代器」，请使用`while let Some(item) = iter.head_next()`
    pub fn head_next(&mut self) -> Option<&T> {
        // 从封装的迭代器中迭代出一个元素
        let item = self.iterator.next();
        // 判断是否结束
        match (self.is_began, item) {
            // 未开始，将要继续 | 第一个元素
            (false, Some(item)) => {
                // 设置「已经开始」
                self.is_began = true;
                // 存入缓冲区
                self.buffer.push_back(item);
                // 头索引不变
                // 取出刚刚置入元素的引用
                Some(self.buffer.back().unwrap()) // * 存入了值
            }
            // 已开始，正在中途
            (true, Some(item)) => {
                // 头索引递增
                self.head += 1;
                // 存入缓冲区
                self.buffer.push_back(item);
                // 取出刚刚置入元素的引用
                Some(self.buffer.back().unwrap()) // * 存入了值
            }
            // 将要结束
            (_, None) => {
                // 设置「已经结束」
                self.is_ended = true;
                None // * 没存入值
            }
        }
        // ! 作为一般的「缓存迭代」，不返回「内置迭代器」迭代出的元素
    }

    /// 缓冲区迭代：从**缓冲区**/**内置迭代器**中拿取元素
    /// * ⚠️总是会拿出元素（故可能涉及缓冲区的索引）
    /// * 🚩分「缓冲区是否为空」执行
    ///   * 缓冲区为空⇒尝试从「内部迭代器」取出元素（调用[`Iterator::next`]）
    ///   * 缓冲区非空⇒从缓冲区头部取出一个元素（先进先出），并返回
    pub fn buffer_next(&mut self) -> Option<T> {
        // 缓冲区为空⇒头迭代（尝试向「内部迭代器」中取）
        if self.is_buffer_empty() {
            // 头迭代，尝试向缓冲区存入元素
            self.head_next();
        }
        // 尝试从缓冲区头部取出元素
        self.buffer.pop_front()
        // ! 此处无需处理「缓冲区头索引」：会自动计算
    }

    /// 头迭代（多次）
    /// * 🚩执行多次头迭代（后续可优化）
    ///   * 返回「是否完全迭代」，即是否`n`次都迭代出了元素
    pub fn head_next_n(&mut self, n: usize) -> bool {
        // 重复n次「头迭代」
        (0..n)
            // 只有所有「头索引步进」都成功时返回true
            .all(|_| self.head_next().is_some())
    }

    /// 缓冲区迭代（多次）
    /// * 🚩不断从**缓冲区**/**内置迭代器**中拿取元素，然后传递进指定的「处理函数」中
    ///   * 单步参见[`Self::buffer_next`]
    pub fn buffer_next_n(&mut self, n: usize, handler: impl Fn(Option<T>)) {
        for _ in 0..n {
            handler(self.buffer_next());
        }
    }

    // ! ❌【2024-03-04 20:58:35】实践：因为「打包后需要从中借用值」的借用问题，再次弃用「独立使用『头迭代器』管理迭代过程」的想法
    // ! ❌【2024-03-04 21:00:13】基于「迭代状态」的「状态机模型」也不可用：「头迭代」「缓冲区迭代」迭代出的是两种不同的类型`T`与`&T`，也没法统一
    // /// 基于「头迭代」生成「头迭代器」
    // /// * 🎯用于「迭代扩充自身缓冲区，并返回『迭代出的元素』的不可变引用」
    // /// * 🚩目前因「头迭代器结构的生命周期问题」**无法直接返回「新加入缓冲区之元素的可变引用」**
    // ///   * 需要结合[`Self::head_item`]或[`Self::head_item_mut`]使用
    // ///
    // /// ! ❌【2024-03-04 20:29:30】基于特殊「头迭代器」的结构无效：无法有效处理生命周期问题
    // /// * ❌「传入自身，然后unwrap解包」的方法：会遇到「返回引用的方法无效」的问题
    // pub fn into_head_iter(self) -> impl Iterator<Item = ()> {
    //     HeadIter::new(self)
    // }
    // ! 弃用：闭包问题
    // pub fn head_iter(&mut self) -> impl Iterator<Item = &T> {
    //     FnIterator::new(|| match self.head_next() {
    //         true => Some(self.head_item().unwrap()),
    //         false => None,
    //     })
    // }

    /// 缓冲区获取
    /// * 📌自缓冲区以**相对位置**索引元素
    ///   * 📌以「缓冲区头索引」为起点（缓冲区头索引=>0）
    /// * 🚩直接获取缓冲区相应位置的元素
    /// * ⚠️越界⇒尝试从「内部迭代器」中取出元素
    ///   * 实在取不到⇒[`None`]
    pub fn buffer_get(&mut self, index: usize) -> Option<&T> {
        match index < self.len_buffer() {
            // * 已经判断了「是否越界」，所以直接进行数组索引
            true => Some(&self.buffer[index]),
            // * 越界⇒尝试扩展缓冲区，并获取头部（缓冲区末尾）元素
            false => match self.head_next_n(index - self.len_buffer() + 1) {
                true => self.head_item(),
                false => None,
            },
        }
    }

    /// 缓冲区迭代器（不可变引用）
    pub fn buffer_iter(&self) -> impl Iterator<Item = &T> {
        self.buffer.iter()
    }

    /// 缓冲区迭代器（可变引用）
    pub fn buffer_iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.buffer.iter_mut()
    }

    /// 缓冲区清空
    /// * 📌「缓冲区头索引」会自动更新
    pub fn buffer_clear(&mut self) {
        self.buffer.clear();
    }

    /// 缓冲区转移（从前往后）
    /// * 🎯在「清空缓冲区」时，需要使用被清空的元素
    /// * 📌其内元素均转移给参数`f`
    /// * 📌「缓冲区头索引」会自动更新
    pub fn buffer_transfer(&mut self, f: impl Fn(T)) {
        // 清除「缓冲区长度」个元素，即清除所有元素
        for _ in 0..self.len_buffer() {
            f(self.buffer.pop_front().unwrap());
        }
    }

    /// 缓冲区转移（从前往后，可变）
    /// * 🎯在「清空缓冲区」时，需要使用被清空的元素，并且过程中会修改其它对象（如「将元素加入某个数组」）
    /// * 📌其内元素均转移给参数`f`
    pub fn buffer_transfer_mut(&mut self, mut f: impl FnMut(T)) {
        // 清除「缓冲区长度」个元素，即清除所有元素
        for _ in 0..self.len_buffer() {
            f(self.buffer.pop_front().unwrap());
        }
    }
}

/// 实现迭代器接口，兼容[`Self::next`]方法
impl<T, I> Iterator for BufferIterator<T, I>
where
    I: Iterator<Item = T>,
{
    type Item = T;

    /// 作为迭代器的迭代：自动重定向到「缓冲区迭代」
    /// * 🎯确保一定会迭代出元素，且无需[`Clone::clone`]复制元素
    /// * 🎯确保其就像一个「正常迭代器」那样迭代
    fn next(&mut self) -> Option<Self::Item> {
        // 重定向到「缓冲区迭代」
        self.buffer_next()
    }
}

/// 对额外实现了[`PartialEq`]的元素实现「以指定迭代元素开头」等方法
impl<T, I> BufferIterator<T, I>
where
    T: Clone + PartialEq,
    I: Iterator<Item = T>,
{
    /// 判断是否以`other_iter`的元素开头
    /// * 🚩从「缓冲区头索引」开始：**优先使用缓冲区内元素**，比对完了再从「内部迭代器」中拿取元素
    ///   * 最多可能新拿取`other_iter.count()`个元素（**比对者长度**）
    /// * 🎯用于在语法解析中实现「前缀匹配」
    /// * ⚠️会改变缓冲区，且不区分「因不匹配而『非前缀』」与「因迭代完而『非前缀』」
    pub fn starts_with(&mut self, mut other_iter: impl Iterator<Item = T>) -> bool {
        // 先比对缓冲区中的元素（不会改变自身） | 此时「比对者」相对未知
        for item_self in &self.buffer {
            // ! ↑此处`item_self`不能加`&`，只需在需要比对时解引用
            // 从「比对者」中取出元素以对比
            match other_iter.next() {
                // 在`false`之前就没有⇒返回`true`
                None => return true,
                // 比对失败⇒返回`false`
                Some(item_other) if *item_self != item_other => return false,
                // 比对成功⇒继续
                _ => {}
            }
        }
        // 再从自身拿出来比对 | 此时「自身」相对未知
        for item_other in other_iter {
            // 从「内部迭代器」中取出元素，置入缓冲区
            match self.head_next() {
                // 然后对比
                // 内部迭代器用尽⇒自身长度不够⇒返回`false`
                None => return false,
                // 迭代出元素⇒从缓冲区中取出元素，对齐，比对
                Some(item_self) => {
                    // 比对失败⇒返回`false`
                    if *item_self != item_other {
                        return false;
                    }
                }
            }
        }
        // 比对都没失败⇒成功⇒`true`
        true
    }

    /// 判断从「『缓冲区头』后i个索引处」开始是否以`other_iter`的元素开头
    /// * ⚠️此处的`i`是相对坐标，0=>缓冲区头，以此类推
    /// * 🎯解析器进行「前缀匹配」不一定在缓冲区头部匹配
    /// TODO: 有待完成
    pub fn starts_with_at(&mut self, i: usize, mut other_iter: impl Iterator<Item = T>) -> bool {
        #![allow(unused)]
        // TODO: 有待完成
        todo!("有待完成！")
    }

    /// 从另一个字符迭代器中返回「缓冲区之后下一个匹配的子串」的开头位置
    /// * 🎯使用「前缀匹配字符串」在识别到「左括弧」后寻找「右括弧」
    /// * 🚩实际上可以直接上暴力算法：不断进行前缀匹配，失败了就挪位，直到匹配成功
    ///   * 💭需要对子串进行缓冲，可能需要构造另一个缓冲区迭代器
    pub fn find_next_substring(&mut self, mut pattern: impl Iterator<Item = T>) -> Option<usize> {
        #![allow(unused)]
        // 先构造子串的缓冲区迭代器
        let pattern = BufferIterator::new(pattern);
        // 然后开始匹配
        // TODO: 有待完成
        todo!("有待完成！")
    }

    /// 若以`other_iter`的元素开头⇒跳过元素
    /// * 🚩仍然会返回「是否 匹配+跳过 成功」
    /// * 📌虽然要求「比对者长度」已知，但「比对者长度」在[`Self::starts_with`]返回`true`时已蕴含「比对者长度已知」
    ///   * 🚩因此使用[`Iterator::map`]封装计数逻辑，并消耗迭代器
    /// * 🚩比对成功后，使用「缓冲区递进」[`Self::buffer_next`]跳过元素
    ///   * 📌因为是从缓冲区开始比对的
    pub fn skip_when_starts_with(&mut self, other_iter: impl Iterator<Item = T>) -> bool {
        let mut c: usize = 0;
        // 使用闭包边迭代边计数（后续用于跳过比对者）
        if self.starts_with(other_iter.map(|v| {
            // 边迭代边计数
            c += 1;
            v
        })) {
            // 使用「缓冲区迭代」跳过比对者
            for _ in 0..c {
                self.next(); // ! 目的在消耗缓冲区内【匹配了前缀】的元素
            }
            // 返回「比对并跳过成功」
            return true;
        }
        // 返回「比对失败」
        false
    }
}

/// 为字符串实现`into_chars`方法
/// * 📄参考：https://internals.rust-lang.org/t/is-there-a-good-reason-why-string-has-no-into-chars/19496/7
pub trait IntoChars {
    /// 将自身转换为字符迭代器，获取自身所有权
    fn into_chars(self) -> impl Iterator<Item = char>;
}

/// 对静态字串实现`into_chars`方法
impl IntoChars for &str {
    fn into_chars(self) -> impl Iterator<Item = char> {
        self.to_owned().into_chars()
    }
}

/// 对动态字串实现`into_chars`方法
impl IntoChars for String {
    /// 迁移自<https://github.com/rust-lang/libs-team/issues/268>
    /// * ⚠️少量修改
    ///   * 🚩使用自己的「函数式迭代器」
    ///   * 📌使用闭包捕获自身作为变量，以避免「临时引用」问题
    fn into_chars(self) -> impl Iterator<Item = char> {
        let mut i = 0;
        // 创建函数式迭代器，捕获变量`i`与自身
        FnIterator::new(move || {
            if i < self.len() {
                let c = self[i..].chars().next().unwrap();
                i += c.len_utf8();
                Some(c)
            } else {
                None
            }
        })
    }
}

/// 单元测试
#[cfg(test)]
mod tests {
    use crate::asserts;

    use super::*;

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

    /// 一次性消耗掉迭代器
    #[test]
    fn iter_char_overview() {
        let test_set = [
            "abcd",
            "我是一个迭代器",
            r"/rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library\std\src\panicking.rs:645",
            "⚠️注意：不能使用`collect`❗，🤔其会获取迭代器的所有权（导致无法知晓「迭代后的状态」）",
        ];
        for test_str in test_set {
            _iter_char_overview(test_str);
        }
    }

    fn _iter_char_overview(s: &str) {
        // ✨创建迭代器
        let mut iter = BufferIterator::new(s.chars());

        // ! ⚠️注意：不能使用`collect`，其会获取迭代器的所有权（导致无法知晓「迭代后的状态」）
        asserts! {
            // 迭代之前
            iter.head() => 0, // 此时头索引为`0`（但实际上是「未开始迭代」的状态）
            iter.is_began() => false, // 还没开始迭代
            iter.is_ended() => false, // 还没终止迭代
            iter.len_buffer() => 0, // 此时缓冲区长度为`0`
            iter.is_buffer_empty(), // 此时缓冲区为空
            iter.buffer_head() => 1, // 此时缓冲区头索引为`1`
        }

        // 一次性迭代完元素
        let mut to = String::new();
        // for _ in &mut head_iter { // ! 弃用「头迭代器」的方式
        while let Some(c) = iter.head_next() {
            // 通过「头迭代」加「获取头元素」实现「不断增扩缓冲区，并返回新增扩的元素的引用」的效果
            to.push(*c); // ! 自动copy了
        }

        // ! 📝字符串长度 ≠ 字符长度（字符个数）
        let len_chars_to = to.chars().count();

        // 迭代之后
        asserts! {
            to => s, // 迭代到字符串中，仍然保持原样
            iter.head() => len_chars_to - 1, // 此时头索引为「字符长度-1」（终态）
            iter.is_began(), // 已经开始迭代
            iter.is_ended(), // 已经终止迭代
            iter.len_buffer() => len_chars_to, // 此时缓冲区长度为「字符长度」
            iter.is_buffer_empty() => false, // 此时缓冲区非空
            iter.buffer_head() => 0, // 此时缓冲区头索引为`0`（因为没消耗缓冲区）
        }

        // 再清空缓冲区
        iter.buffer_clear();

        asserts! {
            iter.head() => len_chars_to - 1, // 此时头索引不变（终态）
            iter.is_began(), // 已经开始迭代
            iter.is_ended(), // 已经终止迭代
            iter.len_buffer() => 0, // 此时缓冲区长度清零
            iter.is_buffer_empty(), // 此时缓冲区为空
            iter.buffer_head() => len_chars_to, // 此时缓冲区头索引为「字符长度」，为空⇔比「头索引」大`1`
        }
    }

    /// 一步步测试迭代器
    #[test]
    fn iter_char_per_step() {
        // ✨创建迭代器
        let mut iter = BufferIterator::new("abcd".chars());

        // ! 尽可能不要尝试在「开始迭代前」获取「头索引」
        asserts! {
            iter.head() => 0 // 此时头索引为`0`（但实际上是「未开始迭代」的状态）
            iter.is_began() => false // 还没开始迭代
            iter.is_ended() => false // 还没终止迭代
            iter.len_buffer() => 0, // 此时缓冲区长度为`0`
            iter.is_buffer_empty(), // 此时缓冲区为空
            iter.buffer_head() => 1 // 此时缓冲区头索引为`1`
        }

        // 迭代器【头迭代】一次 // ! 迭代出的字符【存进缓冲区】，头也【不移动】
        let cached_a = iter.head_next();

        asserts! {
            cached_a => Some(&'a') // 迭代出的字符是'a'
            iter.buffer_get(0) => Some(&'a') // 缓冲区第一个元素为
            iter.head() => 0 // 此时头索引在`0`
            iter.is_began() => true // 此时已开始迭代
            iter.is_ended() => false // 此时未迭代终止
            iter.is_buffer_empty() => false // 此时缓冲区非空
            iter.len_buffer() => 1 // 此时缓冲区长度为`1`
            iter.buffer_head() => 0 // 此时缓冲区头索引在`0`（缓冲区只有第一个）
        }

        // 迭代器【缓冲区迭代】一次 // ! 此时因为缓冲区已缓存，所以缓冲区消耗并返回最前一个字符`'a'`
        let a2 = iter.buffer_next();

        asserts! {
            a2 => Some('a'), // 应该把缓存的第一个字符弹出
            iter.head() => 0, // 此时头索引不变
            iter.is_began() => true, // 此时已开始迭代
            iter.is_ended() => false, // 此时仍未结束
            iter.is_buffer_empty(), // 此时缓冲区为空
            iter.len_buffer() => 0, // 此时缓冲区长度为`0`
            iter.buffer_head() => 1, // 此时「缓冲区头索引」变为`1`
        }

        // 迭代器再次【缓冲区迭代】 // ! 此时因为缓冲区【为空】，所以「内部迭代器」迭代出元素，头索引和缓冲区索引同时移动
        let b = iter.buffer_next();

        asserts! {
            b => Some('b') // 此时没有缓存了，所以迭代出了新字符
            iter.head() => 1 // 此时头索引步进到`1`
            iter.is_began() => true // 此时已开始迭代
            iter.is_ended() => false, // 此时仍未结束
            iter.is_buffer_empty(), // 此时缓冲区为空（本来为空，此时还是空）
            iter.len_buffer() => 0 // 此时缓冲区长度为`0`
            iter.buffer_head() => 2 // 此时「缓冲区头索引」步进到`2`
        }

        // 迭代器通过「缓冲区获取」扩展元素 // ! 此时因为缓冲区【为空】，所以「内部迭代器」迭代出元素，头索引和缓冲区索引同时移动
        let c = iter.buffer_get(0);

        asserts! {
            c => Some(&'c') // 此时没有缓存了，所以迭代出了新字符
            iter.head() => 2 // 此时头索引步进到`1`
            iter.is_began() => true // 此时已开始迭代
            iter.is_ended() => false // 此时仍未结束
            iter.is_buffer_empty() => false // 此时缓冲区非空（因为头索引步进，缓冲区收到了新字符）
            iter.len_buffer() => 1 // 此时缓冲区长度为`1`
            iter.buffer_head() => 2 // 此时「缓冲区头索引」不变
        }

        // 迭代器测试后续是否以"c" "cd" "不会比对成功"开头，在此中将'c'、'd'加入缓冲区
        let starts_with_cd = iter.starts_with("cd".chars());
        let starts_with_c = iter.starts_with("c".chars());
        let starts_with_不会比对成功 = iter.starts_with("不会比对成功".chars());

        asserts! {
            starts_with_cd // 的确是以"cd"开头 | 比对者比缓冲区长
            starts_with_c, // 的确是以"c"开头 | 比对者在缓冲区内
            starts_with_不会比对成功 => false // 的确不以"不会比对成功"开头 | 比对者超出自身界限
            iter.head() => 3 // 此时头索引更新到了`3`——为了「前缀匹配」一直在增加索引
            iter.is_began() => true // 此时已开始迭代
            iter.is_ended() => false // 此时仍未结束 | 临界状态：还未继续调用`next`方法
            iter.is_buffer_empty() => false // 此时缓冲区非空
            iter.len_buffer() => 2 // 此时缓冲区长度为`2`
            iter.buffer_head() => 2 // 此时「缓冲区头索引」不变
        }

        // 测试"c"开头，并（在缓冲区里）跳过它
        let skipped = iter.skip_when_starts_with("c".chars());

        asserts! {
            skipped => true // 的确是以"c"开头并跳过了
            iter.head() => 3 // 此时头索引不变——比对没有超出缓冲区
            iter.is_began() => true // 此时已开始迭代
            iter.is_ended() => false // 此时仍未结束 | 临界状态：还未继续调用`next`方法
            iter.is_buffer_empty() => false // 此时缓冲区非空
            iter.len_buffer() => 1 // 此时缓冲区长度减少到`1`（跳过了"c"）
            iter.buffer_head() => 3 // 此时「缓冲区头索引」增加到`3`（跳过了"c"）
        }

        // 迭代器走到尽头
        let none = iter.head_next();

        asserts! {
            none => None // 已经没有可迭代的了
            iter.head() => 3, // 此时头索引不变
            iter.is_began() // 此时已开始迭代
            iter.is_ended(), // 此时已经结束 | 刚好超过
            iter.is_buffer_empty() => false // 此时缓冲区非空
            iter.len_buffer() => 1 // 此时缓冲区长度不变
            iter.buffer_head() => 3 // 此时「缓冲区头索引」不变
        }

        // 最后的缓冲区转交
        let mut d = String::new();
        iter.buffer_transfer_mut(|c| d.push(c));

        asserts! {
            d => "d" // 转交出来的字符串是"d"
            iter.head() => 3, // 此时头索引不变
            iter.is_began() // 此时已开始迭代
            iter.is_ended() // 此时已经结束
            iter.is_buffer_empty(), // 此时缓冲区为空
            iter.len_buffer() => 0 // 此时缓冲区长度清零
            iter.buffer_head() => 4 // 此时「缓冲区头索引」增加到`4`（为空之后比「头索引」大）
        }
    }
}
