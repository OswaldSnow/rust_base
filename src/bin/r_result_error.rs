use std::fs::File;

fn main(){
    /*
    返回值与错误
     */

    // 不可恢复错误 panic
    // 对于不可恢复的错误，可以调用 panic! 宏抛出异常
    // 会打印出 panic 的位置
    // 默认情况下会进行 栈展开 回溯栈上数据和函数调用
    // 在 main 线程中 panic 程序会直接终止
    // 子线程中 子线程会直接终止 子线程 panic 不会导致程序终止
    panic!("发生了不可恢复错误！");

    // https://course.rs/basic/result-error/panic.html

    use std::net::IpAddr;
    let home: IpAddr = "127.0.0.1".parse().unwrap();


}
