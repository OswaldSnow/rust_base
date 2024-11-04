// use std::fmt::{Debug, Display};

fn main() {
    /*
    trait
     */

    // rust 中的 trait（特征）类似于 java 中的接口
    // trait 也是用来定义行为的
    let p1 = Post::new(
        "xxx事件".parse().unwrap(),
        "cyy".to_string(),
        "xxxxxxx".to_string(),
    );
    println!("{}", p1.summarize());

    println!("p1 title is {}", p1.title);

    // 孤儿规则
    /*
    例如 要为 类型A 实现 特征T
    那么 A 和 T 至少要有一个是在当前作用域内定义的
    比如不能为 String类型 实现 Display特征
    因为无论是 String类型 还是 Display特征 都不是我们自己定义的
    这样确保不会破坏其他代码
     */
    notify(&p1);

    let p2 = returns_post();
    println!("{}", p2.summarize());

    // 使用 try_into 进行类型转换
    // 当使用特征方法时，需要将特征方法引入当前作用域
    // try_into 不需要引入是因为 rust把常用的标准库已经引入到了当前作用域
    let a: i32 = 90;
    let b: u16 = 89;

    let b = b.try_into().unwrap();

    if a > b {
        println!("yes a > b");
    }
}

// 定义一个特征 引用实例本身 返回一个 String
// 此时与具体类型无关 只关注行为
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Post {
    pub title: String,   // 标题
    pub author: String,  // 作者
    pub content: String, // 内容
}

impl Post {
    fn new(title: String, author: String, content: String) -> Self {
        Post {
            title,
            author,
            content,
        }
    }
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!(
            "title is {},author is {},content is {}",
            self.title, self.author, self.content
        )
    }
}

/*
使用特征作为参数
参数是一个 实现了 Summary特征 的类型
这里就类似于 java 的面向接口编程了

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

上面为完整写法
 */
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/*
多种写法
pub fn notify(item1: &impl Summary , item2: &impl Summary)
这种写法 item1 和 item2 不必是同一个类型，可以是：A impl Summary  B impl Summary

pub fn notify<T: Summary>(item1: &T , item2: &T)
这种写法就要求 item1 和 item2 不仅要实现 Summary特征 而且必须为同一类型，都为A 或 都为B

指定多个约束条件
pub fn notify(item: &(impl Display + Summary))

pub fn notify<T: Display + Summary>(item: &T)

当约束条件过多时可以使用 where 约束
pub fn some_function<T: Display + Summary, U: Clone + Debug + Display + Summary>(a: &T,b: &U) -> i32{}

使用 where 改写
pub fn some_function<T,U>(a: &T, b: &U) -> i32
    where T: Display + Summary,
          U: Clone + Debug + Display
{
    println!("{}", a);
    println!("{}", b);
    6
}
 */

/*
返回一个 Trait特征类型
需要注意的是返回的类型虽然只指定了 是一个实现了 Summary特征 类型
但是需要是同一个类型
例如 A、B 两个类型都实现了 Summary

if some_conditions return A
else return B

这种方式是不行的，因为 A 和 B 不是同一种类型
 */
fn returns_post() -> impl Summary {
    Post::new(
        "Title".parse().unwrap(),
        "System".parse().unwrap(),
        "".parse().unwrap(),
    )
}
