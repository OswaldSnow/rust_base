fn main() {
    /*
    模式匹配
     */

    // 普通 match
    let a = 1;
    match a {
        1 => println!("Yes ! It's 1"),
        other => println!("No It's not 1, It's {}", other),
    };

    // if let 匹配集合
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    if let Some(x) = stack.get(0) {
        println!("x is {}", x);
    }

    // while let 循环匹配
    // 不停的从集合从弹出元素 只要是 Some(T) 类型 就执行循环体
    // 此种方式可以循环集合
    while let Some(x) = stack.pop() {
        println!("current val is {}", x);
    }

    // for in 循环
    // enumerate() 返回元组 然后匹配元组
    let v = vec!['a', 'b', 'c'];
    for (index, val) in v.iter().enumerate() {
        println!("当前索引为 {} , 值为 {}", index, val);
    }

    // let a = 'a' let 语句也是一种匹配
    let _a = 'a';
}
