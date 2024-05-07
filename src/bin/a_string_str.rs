fn main(){
    /*
    String 和 str 相关
     */

    // 此时 s 是一个 字符串字面量 并不是 String 类型
    let s = "hello world";
    let hello = &s[..5];
    let world = &s[6..];
    println!("{},{}",hello,world);

    /*
    在 rust 中 字符类型 char 固定 4 个字节
    但是 String 和 &str 是使用的 utf8 每个符号占用 1-3 个字节不等
     */
    //let s = "中国人";
    /*
    切片是按照字节切的
    中 字占用三个字节
    下面这种切法 获取到的 第 1 2 个字节
    会出现错误  而且这种错误编译器无法检查出来 很重要 
     */
    // let s1 = &s[0..2];
    // println!("{}",s1);

    // push 在尾部追加字符或字符串 
    // 会改变原来的 string 需要可变
    let mut s = String::from("hello ");
    s.push_str("world");
    s.push('!');
    
    println!("{}",&s);

    // insert 插入单个字符或字符串
    // 会改变原来的 string 需要可变
    // let mut s = String::from("中国人");
    // /*
    // 此处与前面一样 中 字占3个字节 1 落在 中 的字节中
    // 所以会报错
    //  */
    // s.insert(1, '爱');
    // s.insert_str(3, "爱你");
    // println!("{}",s);

    // replace 替换字符串 (要替换的字符串,新的字符串) 会替换所有匹配到的串
    // 返回一个新的字符串 不改变原有的字符串
    let s = String::from("karima love me , karima love me");
    let s1 = s.replace("me", "cyy");
    println!("{}",s1);

    // replacen (要替换的字符串,新的字符串,替换个数)
    let s2 = s.replacen("me", "cyy", 1);
    println!("{}",s2);

    // replace_range (替换范围,新的字符串) 包头不包尾
    // 会改变原来字符串 需要可变
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..9, "R");
    dbg!(string_replace_range);

    // 与删除有关的方法 pop()，remove()，truncate()，clear()
    // pop() 删除并返回字符串中最后一个字符 返回值为 Option 类型
    let mut string_pop = String::from("rust pop 中文!");
    if let Some(p1) = string_pop.pop(){
        println!("{}",p1);
    }

    // remove() 删除并返回字符串中指定位置的字符 参数必须落在合适的字节位置
    let mut string_remove = String::from("测试remove方法");
    let remove = string_remove.remove(7);
    dbg!(remove);

    // truncate() 删除字符串从指定位置开始到结束的全部字符 参数必须落在合适的字节位置
    // clear() 清空字符串 相当于 truncate 参数为 0 的时候
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);

    // 连接
    // 使用 + 连接，使用 + 实际上是调用了 String 的 add 方法
    // 该方法是 String 实现了 Add trait
    let a1 = String::from("a1 val");
    let a2 = "a2 val";
    let a1_a2 = a1 + a2;
    dbg!(a1_a2);
    // 使用 + 连接后 a1 变量所有权被转移到 add 方法中 方法调用结束后 a1 不再有效
    // println!("{}",a1);
    
    // 宏 format! 连接字符串
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    // format! 不会拿走变量所有权
    println!("{}", s);
    println!("{}",s1);
    println!("{}",s2);

    // 转义
    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# ####in it. And even "##!"###;
    println!("{}", longer_delimiter);

    // 按照字符遍历字符串 这种方式不需要注意字节边界
    for c in String::from("中国人").chars() {
        println!("{}", c);
    }

    // 遍历字节
    for b in "中国人".bytes() {
        println!("{}", b);
    }


}