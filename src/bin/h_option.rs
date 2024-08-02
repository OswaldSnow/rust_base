fn main() {
    /*
    Option
     */

    // Option 枚举表示，一个变量要么有值 Some(T) 要么为 None
    // ”Option 包装了一个值 可以使用模式匹配取出这个值
    let op = Option::Some("oswald snow");
    if let Some(name) = op {
        dbg!(name);
    };

    // 直接使用 unwarap 方法取出包装的值
    // 为 None 会 panic
    let un_op = op.unwrap();
    dbg!(un_op);
}
