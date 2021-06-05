/// 这个部分中，编写了一个宏，如你所见名为 vec_strs ，
/// 在这里我们使用了$e:expr的元变量，用于捕获表达式，同时绑定给$e
/// 在后面，我们通过$e的方式来使用了我们捕获的表达式变量。
/// 同时，我们使用了$(...)sep rep的方式来反复捕获变量。
/// $是标记，()是被反复匹配的内容，sep是可选的分隔标记，可以是,也可以是;
/// 这里我们用的是,
/// 而rep可以使用 ? * + 他们的含义与正则相同，分别表示最多一次，0或者多次，1或者多次 如你所想，当使用?的时候,就不能再使用分割sep标识，否则有冲突。
/// 提问：
///  为什么在后面的=> 中使用了两个{}嵌套？？
/// 为了能够使用多个语句。 （这是原文的解释，但是还是不够清晰）但是可以结合下一个宏看，就可以懂

macro_rules! vec_strs {
    (
        $(
            $element:expr
        ),
        *
    ) => {
        {
            let mut v = Vec::new();
        $(
            v.push(format!("{}",$element));
        )*
        v
        }
    };
}
/// 在这个宏中，我们使用ident用来获取标识符，包括关键字。
/// 在下面的例子中我们捕获到输入的标识符
/// 值得注意的是，当我们尝试使用多组反复捕获时，是必须对称捕获的。看test_repeat_two方法来理解我说的意思。
macro_rules! repeat_two {
    ($($i:ident)*,$($i2:ident)*) =>{
        $(let $i:();let $i2:();)*
    }
}
/// 项目讲述的是将捕获的内容从一个宏传递到另一个宏
/// 第一个宏捕获到内容后，第二个宏看到的是AST片段，他们只能使用相同类型的片段说明符。
/// 但是　ident，lifetime和tt是例外，如下所示
/// ```
/// macro_rules! foo {
///     ($l:expr) => { bar!($l); }
/// // ERROR:               ^^ no rules expected this token in macro call
/// }
///
/// macro_rules! bar {
///     (3) => {}
/// }
///
/// foo!(3);```
///
macro_rules! foo {
    ($l:tt) => {
        bar!($l);
    };
}
macro_rules! bar {
    (3) => {};
}
//1.52 版本的 Rust 已有 13 个片段分类符 (Fragment Specifiers，以下简称分类符)
// item
macro_rules! items {
    ($($item:item)*) => {};
}

items! {
    struct Foo;
    enum Bar {
        Baz
    }
    impl Foo {}
    /*...*/
}
// block
macro_rules! blocks {
    ($($block:block)*) => {};
}

blocks! {
    {}
    {
        let zig;
    }
    { 2 }
}
// stmt
macro_rules! statements {
    ($($stmt:stmt)*) => ($($stmt)*);
}

macro_rules! patterns {
    ($($pat:pat)*) => {};
}

patterns! {
    "literal"
    _
    0..5
    ref mut PatternsAreNice
}
// expr
macro_rules! expressions {
    ($($expr:expr)*) => {};
}

expressions! {
    "literal"
    funcall()
    future.await
    break 'foo bar
}
// ty 类型表达式
macro_rules! types {
    ($($type:ty)*) => {};
}

types! {
    foo::bar
    bool
    [u8]
}
// ident 标识符或者关键字
macro_rules! idents {
    ($($ident:ident)*) => {};
}

idents! {
    // _ /* `_` 不是标识符，而是一种模式 */
    foo
    async
    O_________O
    _____O_____
}
// path 路径
macro_rules! paths {
    ($($path:path)*) => {};
}

paths! {
    ASimplePath
    ::A::B::C::D
    G::<eneri>::C
}
// tt 分类符用于匹配标记树 (TokenTree)。 tt 分类符是最有作用的分类符之一，因为它能匹配几乎所有东西， 而且能够让你在使用宏之后检查 (inspect) 匹配的内容。

//meta 分类符用于匹配属性 (attribute)， 准确地说是属性里面的内容。通常你会在 #[$meta:meta] 或 #![$meta:meta] 模式匹配中 看到这个分类符。
macro_rules! metas {
    ($($meta:meta)*) => {};
}
metas! {
    ASimplePath
    super::man
    path = "home"
    foo(bar)
}
// lifetime 分类符用于匹配生命周期注解或者标签 (lifetime or label)。 它与 ident 很像，但是 lifetime 会匹配到前缀 '' 。
macro_rules! lifetimes {
    ($($lifetime:lifetime)*) => {};
}

lifetimes! {
    'static
    'shiv
    '_
}
// vis 分类符会匹配 可能为空 的内容。 (Visibility qualifier)。 重点在于“可能为空”。你可能想到这是隐藏了 ? 重复操作符的分类符， 这样你就不用直接在反复匹配时使用 ? —— 其实你不能将它和 ? 一起在重复模式匹配中使用。
macro_rules! visibilities {
    // 注意这个逗号，`vis` 分类符自身不会匹配到逗号
    ($($vis:vis,)*) => {};
}

visibilities! {
    ,
    pub,
    pub(crate),
    pub(in super),
    pub(in some_path),
}
// literal 分类符用于匹配字面表达式 (literal expression)。
macro_rules! literals {
    ($($literal:literal)*) => {};
}

literals! {
    -1
    "hello world"
    2.3
    b'b'
    true
}
#[cfg(test)]
mod test {
    /// 元变量 (Metavariables)
    /// 捕获以美元（$）形式写入，其后跟标识符 冒号（:），最后是捕获类型， 也称为 片段分类符 (fragment-specifier)， 片段分类符必须是以下类型之一：
    ///
    /// * block 块：比如用大括号包围起来的语句和/或表达式
    /// * expr 表达式 (expression)
    /// * ident 标识符 (identifier)：包括关键字 (keywords)
    /// * item 条目：比如函数、结构体、模块、impl 块
    /// * lifetime 生命周期注解：比如 'foo、'static
    /// * literal 字面值：比如 "Hello World!"、3.14、'🦀'  所有rust的字面量参见[literal-expr](https://doc.rust-lang.org/reference/expressions/literal-expr.html)
    /// * meta 元信息：指 #[...] 和 #![...] 属性内部的元信息条目
    /// * pat 模式 (pattern)
    /// * path 路径：比如 foo、::std::mem::replace、transmute::<_, int>
    /// * stmt 语句 (statement) [Statements](https://doc.rust-lang.org/reference/statements.html)
    /// * tt：单棵标记树 (single token tree)
    /// * ty 类型
    /// * vis 可视标识符：可能为空的可视标识符，比如 pub、pub(in crate) 参见[Visibility and Privacy](https://doc.rust-lang.org/reference/visibility-and-privacy.html)
    ///
    #[test]
    fn test_macro_show() {
        statements! {
            struct Foo;
            fn foo() {}
            let zig = 3
            let zig = 3;
            3
            3;
            if true {} else {}
            {}
        }
    }

    #[test]
    fn test_vec_strs() {
        let s = vec_strs![1, "a", true, 3.1415];
        assert_eq!(s, &["1", "a", "true", "3.1415"]);
    }

    #[test]
    fn test_repeat_two() {
        repeat_two!( _a _b _c _d _e _f, _u _v _w _x _y _z );
        // 编译报错，可以放开，看看具体的报错。
        // repeat_two!( _a _b _c _d _e _f, _x _y _z );
    }
}

fn main() {
    let s = vec_strs![1, "a", true, 3.1415];
    assert_eq!(s, &["1", "a", "true", "3.1415"]);
}
