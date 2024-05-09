fn main(){
   /*
   枚举类型
    */

    // 简单定义及使用
    let club = PokerSuit::Clubs;
    dbg!(club);

    let _u = (PokerSuit::Hearts,3);
    
    // Option 枚举
    // Option 被设计来替代其他语言中的 null ，所以 rust 中不存在 null 关键字
    // 实际上下面这一行被简化了 完整的写法应该是
    // let a = Option::Some(1);
    let a = Some(3);
    // None 类型也要指定 T 的类型
    let nothing: Option<i32> = None;
    dbg!(a);
    // Option<T> 可以理解为：我预计它可能携带一个 T 类型的值，也可能不携带 T 这个值，这时它就是 None
    match_something(a);
    match_something(nothing);

    if let Some(i) = a {
        println!("解构 Option::Some(t),t is {}",i);
    }



}

fn match_something(x:Option<i32>){
    match x {
        None => {
            println!("this is None!!!");
        },
        Some(val) => {
            println!("this bring something! It's {}",val);
        }
    }
}

#[derive(Debug)]
enum PokerSuit {
    Clubs,
    _Spades,
    _Diamonds,
    Hearts,
}

// 可以直接将数据信息关联到枚举成员上
#[derive(Debug)]
enum PokerCard {
    _Clubs(u8),
    _Spades(u8),
    _ClubsDiamonds(char),
    _Hearts(char),
}

// 枚举成员不不同类型
// 存在普通枚举成员、结构体、元组
enum _Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}