fn main() {
    /*
    生命周期
     */

    // 生命周期 就是 引用的有效作用域

    // 结构体的生命周期
    let data_val = "this is data_val";
    let data1 = Data::new(data_val);
    println!("{:#?}",data1);

    // 生命周期消除
    // 早期版本的 rust 需要为所有引用标注生命周期
    // 但是现在编译器对于可以自行推断的生命周期不需要进行标注
    // 只在编译器无法推断时需要标注生命周期

    // 函数的方法中 参数叫做输入生命周期 返回叫做输出生命周期

    // 三条生命周期消除规则
    // 1、每一个引用类型参数都会获取单独的生命周期
    // 2、若只有一个输入生命周期，那么这个输入生命周期会被赋予给所有的输出生命周期
    // 3、若存在多个输入生命周期，其中一个是 &self 或者 &mut self，那么 &self 的生命周期会赋予给所有的输出生命周期
    // 对于第 3 条，这是编译器的默认行为，如果返回值的生命周期确实不与 &self 一致，那么需要手动标注

    // 生命周期约束语法
    let s1 = "return str lifecycle is 'b";
    let s2 = data1.announce_and_return_part(&s1);
    println!("s2 is: {}",s2);

    // 静态生命周期 'static
    println!("this story is : {}",Data::return_static());


}

// 该引用的生命周期至少与结构体实例本身的生命周期一样长
// 换句话说 被引用的数据的生命周期要 >= 结构体实例本身的生命周期
#[derive(Debug)]
pub struct Data<'a>{
    data: &'a str
}

impl <'a> Data<'a>{
    pub fn new(data: &'a str) -> Self{
        Data{
            data
        }
    }

    pub fn return_some<'b>(&'a self, item: &'b str) -> &'b str{
        return item
    }

    // 生命周期约束语法
    // 'a: 'b, 表示 周期a 要比 周期b 更长
    // 方法的返回值是一个 b周期，但实际返回的是 self.data
    // 所以 self.data 的 周期a 一定要 >= 周期b
    // 符合生命周期约束 'a: 'b
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str
        where 'a: 'b
    {
        println!("Attention please: {}", announcement);
        self.data
    }

    fn return_static() -> &'static str{
        "Karima love OswaldSnow"
    }

}

// 返回值的生命周期与参数生命周期中的较小值一致
// x、y 的生命周期要么 = a 要么 > a
// 在不添加 'a 生命周期标识的情况下
// 编译器无法得知函数在调用时会返回 x引用 还是 y引用
// 就无法得知是否会发生悬垂引用
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 一个综合方法
use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where
        T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


