use std::fmt::Display;

fn main(){
    /*
    深入特征
     */

    /*
    当两个 trait 包含相同的方法
    且某一个结构体同时实现了这两个 trait
    如何确定调用哪一个
     */
    let user1 = User;
    // 带有 &self 的实例方法
    Duck::walk_self(&user1);
    Bird::walk_self(&user1);
    // 不带有 &self 的关联方法
    <User as Duck >::walk();
    <User as Bird>::walk();

    /*
    new type
     */

}

#[derive(Debug)]
pub struct User;
/*
关联类型
 */
pub trait Compare{
    type Item;

    fn compare(&self, item: &Self::Item) -> Self::Item;
}

pub trait Bird{
    fn walk_self(&self);
    fn walk();
}

pub trait Duck{
    fn walk_self(&self);
    fn walk();
}

impl Bird for User{
    fn walk_self(&self) {
       println!("{:?}",self)
    }

    fn walk() {
        println!("User walk by bird")
    }
}

impl Duck for User{
    fn walk_self(&self) {
        println!("{:?}",self)
    }

    fn walk() {
        println!("User walk by Duck")
    }
}

/*
特征的特征约束
含义为 实现此特征的类型必须先实现了 Display 特征
 */
trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}