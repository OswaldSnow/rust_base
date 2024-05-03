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

    // push
    let mut s = String::from("hello ");
    s.push_str("world");
    s.push('!');
    
    println!("{}",&s);

    
}