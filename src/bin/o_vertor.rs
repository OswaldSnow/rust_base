#[allow(unused)]
fn main() {
    /*
    集合类型
     */

    // 动态数据 Vector
    let mut v1: Vec<i32> = Vec::new();
    v1.push(18);

    // 设置容量为 20
    let mut v2: Vec<i32> = Vec::with_capacity(20);
    v2.push(20);

    // vec! 宏
    // 创建时给予初始值
    let v3 = vec![1, 2, 3, 4, 5];

    // 读取元素
    // get 方式返回 Option 类型，可以防止数据越界错误
    let i1 = v3[1];
    let i2 = v3.get(0);
    match i2 {
        Option::Some(t) => {
            println!("val is {}", t)
        }
        None => {
            println!("is None")
        }
    }

    // let mut a = 1;
    // let b = &a;
    // let mut c = &mut a;
    // *c = 10;
    // println!("b val is {b}")

    // 遍历集合
    for i in &v3 {
        println!("val is {}", i);
    }

    for (index, val) in v3.iter().enumerate() {
        println!("index is {index},val is {val}");
    }

    // 常用方法
    let v = vec![1; 5];
    let v_from = Vec::from([1, 2, 3, 4, 5, 6]);

    // 初始化时定义容量
    let mut v = Vec::with_capacity(10);
    v.extend([5, 3, 4, 2, 1]);
    println!("v len is {},capacity is {}", v.len(), v.capacity());

    // 调整大小 在此上下文中会变成 105
    v.reserve(100);
    println!("v len is {},capacity is {}", v.len(), v.capacity());

    // 释放多余的大小 收缩到合适的大小
    v.shrink_to_fit();
    println!("v len is {},capacity is {}", v.len(), v.capacity());

    // 拆分 获取 [下标,len] 位置的数据
    let mut v2 = v.split_off(1);
    println!("{:?}", v2);

    // 排序 默认从小到大
    v2.sort();
    println!("{:?}", v2);

    // 对于自定的的类型进行排序 需要实现一些 trait
    let mut u_vec = vec![
        User::new("Jack".to_string(), 90),
        User::new("Bob".to_string(), 100),
        User::new("Carl".to_string(), 89),
    ];

    u_vec.sort_unstable_by(|a, b| a.age.cmp(&b.age));

    println!("{:?}", u_vec);

    u_vec.sort();
    println!("{:?}", u_vec);
}

// 默认实现排序需要实现的 trait
// 默认实现排序时会按照属性的顺序依次进行比较
#[derive(Debug, Ord, Eq, PartialOrd, PartialEq)]
pub struct User {
    name: String,
    age: u8,
}

impl User {
    pub fn new(name: String, age: u8) -> Self {
        User { name, age }
    }
}
