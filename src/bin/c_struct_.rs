fn main(){
    /*
    结构体
     */

    // 初始化结构体实例
    // 1、每个字段都需要初始化
    // 2、顺序不需要与定义时一致
    let mut user1 = User{
        username:String::from("OswaldSnow"),
        email:String::from("xxxx@xx.com"),
        sign_in_count:1,
        active:true
    };

    // 通过 . 使用
    let _s_count = user1.sign_in_count;

    // 需要修改时 实例需要被标记为可变
    // 不支持将 结构体中的某个字段单独标记为可变
    user1.email = String::from("newEmail@xxx.com");
    user1.username = String::from("OS");
    user1.active = false;

    // 使用函数新建结构体
    let _user2 = build_user("email".to_string(), "username".to_string());

    // 根据已有的结构体实例创建新的结构体实例
    // ..user1 表示其余的字段都从 user1 中取得
    // 且 ..user1 这种必须在最后使用
    let _user3 = User{
        active:false,
        ..user1
    };
    
    // 此时 user1 中的 email username 字段所有权发生转移 user1 不可再被使用
    // 但是 user1 中的其他字段可用

    // let user3 = User{
    //     ..user1
    // };

    println!("{}",user1.active);

    // 元组结构体
    // 结构体本身必须要有名称，但是其中的字段可以没有名称，这种结构体叫做 元组结构体
    let red = Color(255,0,0);
    println!("{}",red.0);
    let _point = Point(100,100,100);

    // 单元结构体
    // 单元结构体只有一个名称 一般用于 需要一个类型，但是不需要内容，只关注行为时
    DoSomething::print_some("Hey It's me !!! Don't fire !!!");

}

struct User{
    active : bool,
    username : String,
    email : String,
    sign_in_count : u64
}

fn build_user(email:String,username:String) -> User{
    // 便捷写法
    User{
        email,
        username,
        active:true,
        sign_in_count:1
    }
}


// 两个元组结构体 表示 颜色、点位置
 struct Color(i32,i32,i32);
 struct Point(i32,i32,i32);

// 定义单元结构体 定义一个方法
#[derive(Debug)]
 struct DoSomething;

 impl DoSomething {
     fn print_some( some : &str){
        dbg!(some);
     }
     
 }