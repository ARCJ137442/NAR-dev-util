# NAR-dev-util

（最后更新：2024-04-05）

服务于上层`Narsese.rs`、`NAVM.rs`、`BabelNAR.rs`等库的**实用开发工具**集

- 🎯提取并统一各个库的`utils`模块（最初用途）
- 🎯可灵活选用的特性组（默认全部启用，亦可条件选用）

主要包含如下实用功能：

- ✨实用宏：大量**开发用语法糖**（总是启用）
  - 🎯测试：批量失败测试、批量断言……
  - 🎯实用语法糖：展示、条件返回……
  - 🎯复杂逻辑表示：截断匹配、张量函数值、平行函数值、管道、操作、for-in-if、列表生成式……
  - 🎯重复表示简化：（带特征条件）模块导入导出……
- ✨浮点：0-1浮点数……
  - 🎯Narsese真值、预算值表示
- ✨字符串处理：前后缀匹配、`join`功能扩展、字符数组切片……
  - 🎯Narsese字符串解析
- ✨迭代器：函数式（弃用）、缓冲区、广度优先遍历……
  - 🎯Narsese字符串解析、依赖图遍历
- ✨`Vec`工具：数组集合操作、搜索算法……
- ✨字符串⇒字符迭代器：`str::into_chars`
- ✨`Option`、`Result`增强：合并、转换（`Err`）
- ✨枚举联合：使用枚举`enum`定义类似TypeScript`A | B | C`的「联合类型」
