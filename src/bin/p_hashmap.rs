use std::collections::HashMap;

fn main() {
    /*
    kv hashmap
     */
    let mut my_gems = HashMap::new();
    my_gems.insert("red gem",9);
    my_gems.insert("diamond",100);
    my_gems.insert("blue gem",88);

    // HashMap::with_capacity(capacity) 也可以提前指定大小

    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    let mut teams_map: HashMap<String,i32> = teams_list.into_iter().collect();
    println!("{:#?}",teams_map);

    // HashMap 中所有权规则与 rust 默认的所有权规则一致
    // 类型没有实现 Copy trait 所有权就会被转移
    // 引用则要注意生命周期

    // get 获取值 是对值的引用
    let china = teams_map.get("中国队");
    match  china{
        Some(val) => {
            println!("{val}");
        }
        _ => {

        }
    }

    teams_map.entry("沙乌地阿拉伯队".to_string()).or_insert(120);
    println!("{:#?}",teams_map);

    let text = "hello world wonderful world the world";

    // 例子 统计每个词在一句话中出现次数
    let mut map = HashMap::new();
    // 根据空格来切分字符串(英文单词都是通过空格切分)
    for word in text.split_whitespace() {
        // 会返回值的可变引用
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);


}