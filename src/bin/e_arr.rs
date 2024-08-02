fn main() {
    /*
    数组 和 集合
     */

    // 数组 array 数组是 rust 的基本类型
    // 长度固定 速度快 存储在栈上
    // 几种初始化方式
    // 直接初始化值
    let _arr = [1, 2, 3, 4, 5, 6];
    // 只给出存储类型和长度
    let _arr: [u8; 3];
    // 初始化简写方式：5个3
    let _arr = [3; 5];

    println!("1st val is {}", &_arr[0]);

    // 小技巧
    // 使用闭包初始化这种数组，不断复制 String
    // 编译器会根据初始化的长度 8 不断调用闭包
    let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));
    println!("{:#?}", array);

    // 数组的切片
    // 与 &str 切片类似 但是数组不需要注意落在有效字节内 直接按照下标进行切片
    let a: [&str; 5] = ["陈冠希", "谢霆锋", "张柏芝", "金城武", "吴彦祖"];
    let slice = &a[0..1];
    dbg!(slice);

    // for in loop
    println!("for index loop");
    for n in 0..a.len() {
        println!("val is {:?}", n);
    }

    println!("for val loop");
    for v in &a {
        println!("val is {:?}", v);
    }

    println!("for in enum loop");
    for (i, v) in a.iter().enumerate() {
        println!("index is {} , val is {}", i, v);
    }
}
