fn main(){
    /*
    模式匹配
     */

    // match 匹配
    // 需要穷举出每一种可能性
    // _ 用于剩下的情况 类似于 java switch 中的 default 
    let s = Direction::South;
    match s {
        Direction::East => println!("east!"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West")
    };

    // match 匹配然后返回赋值
    // 返回值时要保证每一个分支返回的值类型相同
    let cp = Coin::Penny;
    let result = match cp {
        Coin::Nickel => 2,
        _ => 5
    };

    println!("result = {}",result);

    // 或者给一个变量 将其他情况赋值给这个变量
    let result = match cp {
        Coin::Nickel => 2,
        other => {
            println!("other is {:?}",other);
            8
        }
    };

    println!("result = {}",result);


    // if let
    // 匹配单个情况
    let s = Option::Some(6);
    if let Some(x) = s{
        println!("Yes ! It's Same({})",x);
    };

    

    // matches! 宏
    // 数组中存放枚举
    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    let v1: Vec<MyEnum> = v
        .iter()
        .filter(|x| matches!(x, MyEnum::Foo))
        .cloned()
        .collect();
    dbg!(v1);

    // 后面可以跟一个表达式
    let bar = Some(9);
    let _bar_10 =  matches!(bar,Some(x) if x < 10 );

    // 匹配范围
    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

    // 变量遮罩 类似于作用域的概念
    let age = Some(30);
    println!("{:?}",age);
    if let Some(age) = age{
        println!("{:?}",age);
    }
    println!("{:?}",age);


}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}

#[derive(Debug,Clone)]
enum MyEnum {
    Foo,
    Bar
}
