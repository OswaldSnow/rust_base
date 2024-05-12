fn main(){
    /*
    方法
     */

    // rust 中对象的定义和方法的定义是分离的
    let rec = Rectangle::new(10, 20);
    println!("rec area is {}",rec.area());

    // 携带 self 一般是实例方法 使用 实例. 调用
}

// 定义结构体
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}

// 为结构体实现方法
impl Rectangle{

    // &self 其实含义是 self: &Self 表示当前实例
    // &self 不可变引用
    // &mut self 可变引用
    // self 使用时所有权会转移
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn new(width: u32,height: u32) -> Self{
        Rectangle { width, height }
    }
}