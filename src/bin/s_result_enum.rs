use std::fs::File;
use std::io::Error;

#[allow(unused)]
fn main() {
    /*
    Result<T,E> 枚举
     */

    let file : Result<File,Error>= File::open("hello.txt");
    match file {
        Ok(file) => {
            println!("file name is {:?}",file);
        },
        Err(error) => {
            println!("error is {:?}",error);
        }
    }

    // 自定义返回 Result 方法
    let res = divide(19,9);

    /*
    失败就 panic: unwrap 和 expect
    使用 unwrap 和 expect
    正常时就讲 Ok(T) 中的 T 值返回
    不正常就默认打印出错误信息
    二者都会将具体的错误信息输出
    但是 expect 可以添加一些自定义的输出
     */
    // let file = File::open("hello.txt").unwrap();

    let file = File::open("hello.txt").expect("打开文件出现错误");

    // 错误传播
    // ?



}

// 随便自定义一个 Result
fn divide(x: i32, y: i32) -> Result<i32, u8> {
    if y == 0 {
        // 除数为 0,返回错误
        Err(1) // 1 可以是任何 u8 类型的值,表示特定的错误码
    } else {
        // 计算结果并返回
        Ok(x / y)
    }
}