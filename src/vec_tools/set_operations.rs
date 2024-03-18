//! 主要定义一些数组用的「集合操作」
//! * 用于对数组「取交集」「取并集」等

/// 工具函数：两个向量取并集
pub fn set_union_vec<'val, 'arr, T>(vec1: &'arr [T], vec2: &'arr [T]) -> Vec<&'val T>
where
    'arr: 'val,
    T: PartialEq + 'val,
{
    let mut result = vec![];
    // 非重复添加
    for v1 in vec1 {
        match result.iter().find(|&&v| v == v1) {
            Some(..) => {}
            None => result.push(v1),
        }
    }
    // 非重复添加
    for v2 in vec2 {
        match result.iter().find(|&&v| v == v2) {
            Some(..) => {}
            None => result.push(v2),
        }
    }
    result
}

/// 工具函数：两个向量判子集
/// * 🚩子集的所有元素都包含于超集之中
pub fn set_is_subset<'val, 'arr, T>(sub: &'arr [T], sup: &'arr [T]) -> bool
where
    'arr: 'val,
    T: PartialEq + 'val,
{
    // 💭【2024-03-02 10:28:00】实质上还是两层循环
    sub.iter()
        .all(|sub_value| 
            // 内层：只要有一个，就算「包含在内」
            sup.iter()
                .any(|sup_value| 
                    sub_value == sup_value
                )
        )
}

/// 工具函数：两个向量判非空交
/// * 🚩交集非空
pub fn set_has_intersection<'val, 'arr, T>(s1: &'arr [T], s2: &'arr [T]) -> bool
where
    'arr: 'val,
    T: PartialEq + 'val,
{
    // 💭【2024-03-02 10:28:00】实质上还是两层循环
    s1.iter()
        // 外层：只要有一个包含在`s2`内，就算「有交集」
        .any(|sub_value| 
            // 内层：只要有一个，就算「`s1`的也包含在内」
            s2.iter()
                .any(|sup_value| 
                    sub_value == sup_value
                )
        )
}