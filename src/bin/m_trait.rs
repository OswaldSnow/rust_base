fn main(){
    /*
    trait
     */

    // rust 中的 trait（特征）类似于 java 中的接口
    // trait 也是用来定义行为的
    let p1 = Post::new("xxx事件".to_string(), "cyy".to_string(), "xxxxxxx".to_string());
    println!("{}",p1.summarize());
    
    println!("p1 title is {}",p1.title);

    // 孤儿规则
    // to be continue 
}

// 定义一个特征 引用实例本身 返回一个 String
// 此时与具体类型无关 只关注行为
pub trait Summary { 
    fn summarize(&self) -> String;
}

pub struct Post {
    pub title: String, // 标题
    pub author: String, // 作者
    pub content: String, // 内容
}

impl Post{
    fn new(title: String,author: String,content: String) -> Self{
        Post { title, author, content }
    }
}

impl Summary for Post{
    fn summarize(&self) -> String {
        format!("title is {},author is {},content is {}",self.title,self.author,self.content)
    }
}
