/// 用于**从「枚举变种」的角度**关联「枚举联合」
/// * 🎯配合「枚举联合」类型的如下方法使用
///   * 判别：`is_variant::<子类型>()`
///   * 向下转换：`try_into_variant::<子类型>()`
/// * 🚩类似[`TryFrom`]，但仅返回布尔值
/// * 🚩【2024-03-31 22:17:30】现在功能扩大，以替代无法直接实现的[`TryFrom<XXX<T>>`]
///   * ⚠️后者的`impl`会触发``type parameter `T` must be covered by another type when it appears before the first local type``
pub trait VariantTypeOf<EnumUnion> {
    /// 某个「枚举联合」的变种是否为其值
    fn is_variant_type_of(union: &EnumUnion) -> bool;

    /// 尝试从某个「枚举联合」转换为（作为变种之一的）当前类型
    fn try_from_variant(union: EnumUnion) -> Option<Self>
    where
        Self: Sized;
}

/// 「枚举联合」
/// * 🎯用于快速**利用已有类型**定义「类型联合体」
/// * ✨类似TypeScript中`type Union = A | B | C;`的语法
/// * ✨自动提供「类型判断」「向上转换」「向下转换」方式
///   * 📌「类型判断」通过自身**与子类型同名**的方法进行判断，返回布尔值[`bool`]
///   * 📌「向上转换」直接在子类型上调用`.into::<联合类型>()`即可
///     * ✅联合类型自动实现了`From<子类型>`特征
///   * 📌「向下转换」通过在联合类型上调用`.try_into_variant<子类型>()`
///     * ✅子类型自动实现了`VariantTypeOf<联合类型>`特征
/// * ✨可见性注释、文档注释、属性宏仍然有效
/// * 📝【2024-03-31 22:05:39】学习笔记：无泛型版本实现起来很简单，然而一旦需要支持泛型，就会变得非常复杂
///   * 🚩（不得已）使用方括号容纳泛型参数，以避免匹配的「本地歧义」
///   * ✅【2024-03-31 22:30:23】基本支持泛型类型
#[macro_export]
macro_rules! enum_union {
    // 类枚举语法`enum 自身 { 子类型1,子类型2,子类型3, .. }`
    {
        // * 📝↓下边这个`vis`可以匹配空标签树
        $(#[$m:meta])*
        $v:vis $name:ident $( [ $($generics_self:tt)* ] )? {
            $(
                $variant:ident $(,)? $( [ $($generics:tt)* ] )?
            )+
        }
    } => {
        $crate::enum_union!{
            $(#[$m])*
            $v $name $( [ $($generics_self)* ] )?
            = $( $variant $( [ $($generics)* ] )? )|+
        }
    };
    // 类TypeScript语法`type 自身 = 子类型1 | 子类型2 | 子类型3 | .. ;`
    // * 📌分号可选
    {
        // * 📝↓下边这个`vis`可以匹配空标签树
        $(#[$m:meta])*
        $v:vis $name:ident $( [ $($generics_self:tt)* ] )?
        = $( $variant:ident $( [ $($generics:tt)* ] )? )|+ $(;)?
    } => {
        // * 📝↓下边这个`vis`可以匹配空标签树
        $crate::enum_union! {
            @INNER
            $(
                $variant [ $( $( $generics )* )? ]
            )+ // ! 把「变种的重复」放在前，后边就可以用「标签流」省略
            =>
            // 各变种实现 | 在此预先展开
            // ! 🚩避免下边「变种展开」与「`tail`展开」冲突，把`tail`封装起来
            {
                $(#[$m])*
                $v $name [ $( $( $generics_self )* )? ]
            }
        }
    };
    // // 泛型参数展开
    // {
    //     @EXPAND_GENERICS []
    // } => {};
    // {
    //     // 泛型参数展开
    //     @EXPAND_GENERICS [ $($generics_self:tt)* ]
    // } => {
    //     $($generics_self)*
    // };
    // 归一化语法
    // * 🎯用于处理「泛型参数问题」和「广播展开」问题
    // * 🚩最终展开用
    //   * 使用`[]`取消掉一层「可省略的泛型参数」语法
    //     * 🎯避免再重复一层`?`
    //   * ❌原先无分隔的变种，加了泛型之后会「本地歧义」
    //   * ❗现在因为「重复次数歧义（没有『广播规则』）」的需要，得再次转发
    {
        @INNER
        // // // * 📝↓下边这个`vis`可以匹配空标签树
        // $(#[$m:meta])*
        // $v:vis $name:ident $generics_self:tt // ! ←此处延迟展开
        // = $( $variant:ident [ $($generics:tt)* ] )+
        // $($token:tt)*
        $(
            $variant:ident [ $( $generics:tt )* ]
        )+ // ! 把「变种的重复」放在前，后边就可以用「标签流」省略
        => $tail:tt
    } => {
        // 枚举定义
        $crate::enum_union! {
            @ENUM
            // ↓这俩都是原样怼回去
            $( $variant [ $( $generics )* ] )+
            => $tail
        }
        // 各变种实现 | 在此预先展开
        // ! 🚩避免此处与`tail`展开冲突，把`tail`封装起来
        $(
            $crate::enum_union! {
                @VARIANT
                $variant [ $( $generics )* ]
                => $tail // ! 就是此处需要封装，否则会与「变种重复」产生冲突
            }
        )+
    };
    // 实现其中有关「枚举定义」的部分
    {
        @ENUM
        // * 📝↓下边这个`vis`可以匹配空标签树
        $( $variant:ident [ $($generics:tt)* ] )+
        => {
            $(#[$m:meta])*
            $v:vis $name:ident [ $($generics_self:tt)* ] // ! ←此处延迟展开
        }
    } => {
        // `enum`
        $(#[$m])* $v enum $name < $($generics_self)* > {
            $(
                $variant($variant < $($generics)* > ),
            )+
        }
        // `impl enum`
        impl < $($generics_self)* > $name < $($generics_self)* > {
            // $(
            //     // ! 🚩【2024-03-31 19:35:50】↓现在还没法加进文档字符串
            //     // #[doc = concat!("判断是否为", stringify!($variant), "变种")]
            //     // 目前还没法把标识符连接起来：功能不稳定
            //     // * 🚩所以只好直接占用标识符名称（其它用`try_from`弥补）
            //     // * 🔗<https://doc.rust-lang.org/std/macro.concat_idents.html>
            //     /// 判断自身是否为某个变种
            //     #[inline]
            //     #[allow(non_snake_case)] // ! 需要允许非蛇形命名
            //     pub fn $variant(self) -> bool {
            //         matches!(self, Self::$variant(..))
            //     }
            // )+
            // ! ↑🚩【2024-03-31 20:32:01】不再需要：使用「外置特征」的方法解决

            /// 判断自身是否为某个子类型
            /// * 🚩利用批量实现的`is_variant_type_of`方法
            #[allow(non_camel_case_types)] // ! 使用`r#type`尽可能避免名称占用
            pub fn is_variant<r#type>(&self) -> bool
                where r#type: VariantTypeOf<Self>
            {
                r#type::is_variant_type_of(self)
            }

            /// 判断自身类型是否与另一个值相同
            /// * 🚩在`match`中重复实现模式
            #[allow(non_camel_case_types)] // ! 使用`r#type`尽可能避免名称占用
            pub fn eq_variant(&self, other: &Self) -> bool
            {
                match (self, other) {
                    // * 📝使用`|`节省多余的匹配臂数目
                    $(
                        (Self::$variant(..), Self::$variant(..))
                    )|+ => true,
                    _ => false,
                }
            }

            /// 尝试将自身转换为某个子类型
            /// * 🚩利用自身的`try_into`方法
            ///   * 若自身是某个子类型，则返回`Some(子类型对象)`
            ///   * 若自身不是某个子类型，则返回`None`
            #[inline]
            #[allow(non_camel_case_types)]
            pub fn try_into_variant<r#type>(self) -> Option<r#type>
                // * ↓此处需要如此约束
                where r#type: VariantTypeOf<Self>
            {
                VariantTypeOf::<Self>::try_from_variant(self)
            }
        }
    };
    // 实现其中有关「各变种实现」的部分
    // * 在调用前就重复了「变种泛型」
    {
        @VARIANT
        // * 📝↓下边这个`vis`可以匹配空标签树
        // * ✅【2024-03-31 22:06:55】↓已经预先展开单态化
        $variant:ident [ $($generics:tt)* ]
        => {
            $(#[$m:meta])*
            $v:vis $name:ident [ $($generics_self:tt)* ] // ! ←此处延迟展开
        }
    } => {
        // 实现自身与子类型的转换接口
        /// 自动实现`From<子类型>`特征
        /// * 🚩直接使用自身的同名变种封装
        impl < $($generics_self)* > From<$variant < $($generics)* > > for $name < $($generics_self)* > {
            fn from(v: $variant < $($generics)* > ) -> Self {
                Self::$variant(v)
            }
        }

        /// 自动为子类型实现`TryFrom<自身>`特征
        /// * 🚩直接使用自身的同名变种进行匹配
        // impl < $($generics_self)* > TryFrom<$name < $($generics_self)* > > for $variant < $($generics)* > {
        //     /// 错误类型为空
        //     /// * 只有唯一语义：不是该子类型
        //     type Error = ();

        //     fn try_from(v: $name < $($generics_self)* > ) -> Result<Self, Self::Error> {
        //         match v {
        //             // 是类型⇒转换成功
        //             $name::$variant(v) => Ok(v),
        //             // 不是类型⇒转换失败
        //             _ => Err(()),
        //         }
        //     }
        // }

        impl < $($generics_self)* > $crate::enum_union::VariantTypeOf<$name < $($generics_self)* > > for $variant < $($generics)* > {
            fn is_variant_type_of(union_value: &$name < $($generics_self)* > ) -> bool {
                matches!(union_value, $name::$variant(..))
            }

            fn try_from_variant(union_value: $name < $($generics_self)* > ) -> Option<Self>
            where
                Self: Sized
            {
                match union_value {
                    // 是类型⇒有值
                    $name::$variant(v) => Some(v),
                    // 不是类型⇒无值
                    _ => None,
                }
            }
        }
    }
}

/// 单元测试
#[cfg(test)]
mod tests {
    // #![allow(unused)]
    use super::*;
    use crate::*;
    use std::collections::HashSet;

    /// 测试/普通`enum`语法
    #[test]
    fn test_enum() {
        enum_union! {
            /// 枚举类型的「有符号整数」
            #[allow(non_camel_case_types)]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
            Int {
                i8,
                i16,
                i32,
                i64,
                i128,
            }
        }

        // 向上转换
        let i = Int::i32(32);
        let i2: Int = 32_i32.into();

        // 测试
        asserts! {
            // 判断其变种
            i.is_variant::<i32>(),

            // 从变种到内容均相等
            i.eq_variant(&i2),
            i == i2,

            // 向下转换
            i.try_into_variant::<i32>() => Some(32),
            i64::try_from_variant(i) => None,
        }
    }

    /// 类TS语法
    #[test]
    fn test_ts() {
        enum_union! {
            /// 枚举类型的「无符号整数」
            #[allow(non_camel_case_types)]
            #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
            Float = f32 | f64;
        }

        // 向上转换
        let f: Float = 0.1_f64.into();
        let f2 = Float::from(0.1_f32);

        asserts! {
            // 值相同，但类型不同
            f != f2,
            !f.eq_variant(&f2),

            // 向下转换
            f.is_variant::<f64>(),
            !f.is_variant::<f32>(),
            f.try_into_variant() != Some(0.1_f32),
            f.try_into_variant::<f32>() => None,
            f.try_into_variant::<f64>() => Some(0.1_f64),

            f.try_into_variant::<f64>().unwrap() => f2.try_into_variant::<f32>().unwrap() as f64
        }
    }

    /// 泛型语法
    #[test]
    fn test_generics() {
        enum_union! {
            /// 基于泛型类型的枚举类型
            /// * 🚩泛型参数使用**方括号**而非尖括号
            ///   * 📌是出于「尖括号并非括号，导致宏匹配『本地歧义』」的妥协
            #[derive(Debug, Clone)]
            Container[T] =
                String // 允许不带泛型参数
              | Option[T]
              | Vec[T] // 一个泛型类型参数
              | HashSet[T]
        }

        /// ✨展示：实际实现
        impl<T> Container<T> {
            /// ✨展示：实际使用匹配的情况
            pub fn capacity_description(&self) -> &str {
                match self {
                    Self::String(..) => "many chars",
                    Self::Option(..) => "0~1",
                    Self::Vec(..) => "many @ vec",
                    Self::HashSet(..) => "many @ set",
                }
            }
        }

        // 向上转换
        type Cu = Container<usize>;
        let c: Cu = "container".to_string().into();
        let c2 = Cu::from(vec![0usize]);

        asserts! {
            // 基于类型的比较
            c.is_variant::<String>(),
            c.eq_variant(&c),
            c2.eq_variant(&c2),
            !c.eq_variant(&c2),

            // 向下转换
            c.capacity_description() => "many chars",
            c.clone().try_into_variant() != Some(Some(0)),
            c.clone().try_into_variant::<Vec<_>>() => None,
            c.clone().try_into_variant::<String>() => Some("container".into()),

            c2.capacity_description() => "many @ vec",
            c2.clone().try_into_variant() != Some(Some(0)),
            c2.clone().try_into_variant() != Some(None),
            c2.clone().try_into_variant::<Vec<_>>().unwrap()[0] => 0,
            c2.clone().try_into_variant::<Vec<_>>() => Some(vec![0]),
            c2.clone().try_into_variant::<String>() => None,
        }
    }
}
