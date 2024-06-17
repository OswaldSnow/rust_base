// 同时存在 main.rs 和 lib.rs 的项目
// 从 库根中使用 mod
use rust_base::utils;
use rust_base::Person;

// 使用第三方库
// 引入时使用 self
// 表示 引入当前模块本身 和 模块的 Rng
use rand::{self,Rng};
// 引入方式
// 以下方式可能会引入一些与当前作用域名称冲突的东西
// use rand::*;

fn main(){
    println!("Hello, world!");

    // Use the function from the library crate
    let sum = utils::add(2, 3);
    println!("2 + 3 = {}", sum);

    // Create an instance of the struct from the library crate
    let person = Person::new("Alice", 30);
    println!("Name: {}, Age: {}", person.name, person.age);

    // 使用第三方库获取指定范围内随机值
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("secret_number is {secret_number}");

}
