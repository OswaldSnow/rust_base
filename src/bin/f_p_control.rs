fn main() {
    /*
    流程控制
     */

    // if
    let condition = true;
    if condition == true {
        println!("condition is true");
    }

    // rust 的 三元运算符 🐶
    let number = if condition { 1 } else { 0 };
    dbg!(number);

    // for in
    for i in 0..=5 {
        println!("i={}", i);
    }

    let mut a = [1, 2, 3, 4, 5, 6];
    dbg!(a);
    for v in &mut a {
        *v = 3;
    }
    dbg!(a);

    // while
    let mut n = 1;
    while n <= 5 {
        dbg!(n);
        n += 1;
    }

    // loop 使用 break 结束
    let mut n = 1;
    loop {
        if n > 5 {
            break;
        }
        println!("{}", n);
        n += 1;
    }

    // 注
    // break 可以单独使用，也可以带一个返回值，有些类似 return
    // loop 是一个表达式，因此可以返回一个值
    let mut counter = 1;
    let result = loop {
        counter += 1;
        if counter >= 10 {
            break counter * 2;
        }
    };
    dbg!(result);
}
