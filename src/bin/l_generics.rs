use std::fmt::Display;

fn main() {
    /*
    泛型
     */
    Point::<i32>::print_self_name();

    let p1 = Point { x: 1, y: 2 };
    println!("x is {},y is {}", p1.x(), p1.y());

    let p2 = Point::new(89, 100);
    println!("x is {},y is {}", p2.x(), p2.y());

    // const 泛型
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32; 2] = [1, 2];
    display_array(arr);

    // something([0u8; 0]); // ok
    // something([0u8; 512]); // ok
    // something([0u8; 1024]); // 编译错误，数组长度是1024字节，超过了768字节的参数长度限制

    /*
    rust 泛型是零成本抽象
    写代码时候我们抽象的指定 T:someTrait
    但是编译时 rust 会为每一种类型实现对应的方法
     */
}

// 最基本用法
struct Point<T> {
    x: T,
    y: T,
}

// 为泛型结构体实现方法
impl<T: Display> Point<T> {
    fn print_self_name() {
        println!("my name is Point<T>")
    }
}

// 对特定的泛型类型实现方法
impl Point<i32> {
    fn new(x: i32, y: i32) -> Point<i32> {
        Point { x, y }
    }

    fn x(&self) -> &i32 {
        &self.x
    }

    fn y(&self) -> &i32 {
        &self.y
    }
}

// const 泛型
// 针对值的泛型 可以理解为针对具体类型的泛型
fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

// // 初看时没理解的代码
// // 目前 rust 版本(1.73.0) 不支持 core::mem::size_of::<T>() < 768 写法
// // 但是需要理解以下代码
// fn something<T>(val: T)
// where
//     Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
//     //       ^-----------------------------^ 这里是一个 const 表达式，换成其它的 const 表达式也可以
// {
//     //
// }

// pub enum Assert<const CHECK: bool> {
//     //
// }

// pub trait IsTrue {
//     //
// }

// impl IsTrue for Assert<true> {
//     //
// }
