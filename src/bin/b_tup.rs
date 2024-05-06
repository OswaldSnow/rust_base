fn main(){
   /*
   元组相关
    */

    // 定义元组
    let tup1 = (500,6.4,1,String::from("CYY"));
    // 使用 模式匹配 或者 . 获取元素 可以使用 _ 忽略某个元素
    let (x,y,z,_) = tup1;
    let n = tup1.3;
    println!("{},{},{},{}",x,y,z,n);

    // 函数调用 元组作为返回值
    let s1 = String::from("hello");
    let (s1, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s1, len);

}

// 在函数返回值场景下很常用
// 一下函数返回输入的 字符串和其长度
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}