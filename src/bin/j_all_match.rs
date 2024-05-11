#[allow(unused_variables)]
fn main(){
    /*
    全模式匹配列表
     */

    /*
    很简单 没有必要拿过来的都在
    https://course.rs/basic/match-pattern/all-patterns.html 上了
    可以去温习
     */

     // 数组可以如下匹配
     let arr: [u16; 2] = [114, 514];
     let [x, y] = arr;
     
     assert_eq!(x, 114);
     assert_eq!(y, 514);
     
     let arr: &[u16] = &[114, 514];

    if let [x, ..] = arr {
        assert_eq!(x, &114);
    }

    if let &[.., y] = arr {
        assert_eq!(y, 514);
    }

    let arr: &[u16] = &[];

    assert!(matches!(arr, [..]));
    assert!(!matches!(arr, [x, ..]));

    // 忽略模式
    foo(3,4);

    // 匹配守卫的额外条件
    // 可以在匹配值的后面添加条件判断
    let num = Some(6);
    match num {
        Some(x) if x >= 6 => println!("yes it's large than 6"),
        Some(x) => println!("no it's small than 6"),
        None => println!("it's None")
    }

    // @绑定
    // 匹配 msg 是否是一个 Message::Hello 类型的值 并且 id 在 3到7 范围内
    // 然后 如果我们在匹配到时使用当前的 msg 的 Hello 的 id 值
    // 就需要使用到 @ 绑定 将 id 的值绑定到变量 id_val 中
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id : id_val @ 3..=7 } => {
            println!("id is {}" , id_val);
        },
        _ => ()
    }

    // 前绑定后结构
    // 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point {x: px, y: py } = Point {x: 10, y: 23};
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);

    // 特性
    // match n 
    // 先看右边 匹配：当 n = 1 或者 n = 2 时候
    // 将匹配到的值（这里其实就是n）绑定到变量 num 上，然后取用
    let n = 2;
    match n {
        num @ (1 | 2) =>  {
            println!("num is {}",num);
        },
        _ => ()
    }

}

// 忽略模式 的函数
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

enum Message {
    Hello { id: i32 },
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}