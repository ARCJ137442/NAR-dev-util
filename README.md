# NAR-dev-utils

    ！项目英文文档有待增加
    ! The English documentation is still not completed

🕒最后更新：2024-09-14

<!-- 徽章安排参考：https://daily.dev/blog/readme-badges-github-best-practices#organizing-badges-in-your-readme -->

![License](https://img.shields.io/crates/l/nar_dev_utils?style=for-the-badge&color=ff7043)
![Code Size](https://img.shields.io/github/languages/code-size/ARCJ137442/NAR-dev-utils?style=for-the-badge&color=ff7043)
![Lines of Code](https://www.aschey.tech/tokei/github.com/ARCJ137442/NAR-dev-utils?style=for-the-badge&color=ff7043)
[![Language](https://img.shields.io/badge/language-Rust-orange?style=for-the-badge&color=ff7043)](https://www.rust-lang.org)

<!-- 面向用户 -->

Cargo状态：

[![crates.io](https://img.shields.io/crates/v/nar_dev_utils?style=for-the-badge)](https://crates.io/crates/nar_dev_utils)
[![docs.rs](https://img.shields.io/docsrs/narust-158?style=for-the-badge)](https://docs.rs/nar_dev_utils)
![Crate Size](https://img.shields.io/crates/size/nar_dev_utils?style=for-the-badge)

![Recent Downloads](https://img.shields.io/crates/dr/nar_dev_utils?style=for-the-badge)
![Downloads](https://img.shields.io/crates/d/nar_dev_utils?style=for-the-badge)
![Crates.io Dependents](https://img.shields.io/crates/dependents/nar_dev_utils?style=for-the-badge)

<!-- 面向开发者 -->

开发状态：

[![CI status](https://img.shields.io/github/actions/workflow/status/ARCJ137442/NAR-dev-utils/ci.yml?style=for-the-badge)](https://github.com/ARCJ137442/NAR-dev-utils/actions/workflows/ci.yml)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-2.0.0-%23FE5196?style=for-the-badge)](https://conventionalcommits.org)
![GitHub commits since latest release](https://img.shields.io/github/commits-since/ARCJ137442/NAR-dev-utils/latest?style=for-the-badge)

![Created At](https://img.shields.io/github/created-at/ARCJ137442/NAR-dev-utils?style=for-the-badge)
![Last Commit](https://img.shields.io/github/last-commit/ARCJ137442/NAR-dev-utils?style=for-the-badge)

## 简介

服务于上层`Narsese.rs`、`NAVM.rs`、`BabelNAR.rs`等库的**实用开发工具**集

- 🎯提取并统一各个库的`utils`模块（最初用途）
- 🎯可灵活选用的特性组（默认全部启用，亦可条件选用）

## 主要功能

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
- ✨`Void`特性：销毁表达式以简化类 `{ expr; }` 即 `()` 的语法
- ✨「引用计数」功能接口：统一表示 `Rc<RefCell<T>>` 与 `Arc<Mutex<T>>` 等「共享引用」类型
