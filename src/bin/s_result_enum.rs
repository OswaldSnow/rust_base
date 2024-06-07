use std::fs::File;
use std::io::Error;
use std::io;
use std::io::Read;

#[allow(unused)]
fn main() -> Result<(), Box<dyn std::error::Error>>  {
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
    let file_res_str = read_username_from_file();


    // main 函数中的传播
    let f = File::open("hello.txt")?;
    Ok(())

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

// ? 传播 函数示例
// 在可能抛出异常的代码后使用 ? 可以省略 match 对于错误的匹配
fn read_username_from_file() -> Result<String, io::Error> {
    // 这段含义是 如果代码正常执行 将返回值给 f
    // 如果出现异常 将异常直接 return
    // ? 操作符需要一个变量来承载正确的值
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 链式调用
#[allow(unused)]
fn read_username_from_file_chain() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// Option 也可以使用 ? 传播
#[allow(unused)]
fn first(arr: &[i32]) -> Option<&i32> {
    // arr.get(0)

    let v = arr.get(0)?;
    Some(v)

    // 以下代码编译不通过
    // 只返回了出现异常的值 None
    // 没有对正常的值处理
    // 这种语法正常时 不会将正常的值返回
    // 需要 一个变量来承载正确的值
    // arr.get(0)?

}
