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