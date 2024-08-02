use std::fmt::{Display, Formatter};

fn main() {
    /*
    格式化
     */
    println!("Hello"); // => "Hello"
    println!("Hello, {}!", "world"); // => "Hello, world!"
    println!("The number is {}", 1); // => "The number is 1"
    println!("{:?}", (3, 4)); // => "(3, 4)"
    println!("{value}", value = 4); // => "4"
    println!("{} {}", 1, 2); // => "1 2"
    println!("{:04}", 42); // => "0042" with leading zeros

    // 输出到标准错误输出
    eprintln!("something wrong!!!");

    // Display 和 Debug 特征
    // {} 需要实现 Display 特征
    // {:?} {:#?} 需要实现 Debug特征 或 派生Debug
    let u1 = User::default();
    println!("Display: {}", u1);
    println!("Debug: {:#?}", u1);

    // 位置参数
    println!("{}{}", 1, 2); // =>"12"
    println!("{1}{0}", 1, 2); // =>"21"
                              // => Alice, this is Bob. Bob, this is Alice
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{1}{}{0}{}", 1, 2); // => 2112

    // 为参数指定名称
    println!("{argument}", argument = "test"); // => "test"
    println!("{name} {}", 1, name = 2); // => "2 1"
    println!("{a} {c} {b}", a = "a", b = 'b', c = 3); // => "a 3 b"

    // 带名称的要放在不带名称的后面 不然会出错
    // Positional arguments must come before named arguments
    // println!("{abc} {1}", abc = "def", 2);

    // 格式化输出
    let v = std::f64::consts::PI;
    // 保留小数点后两位
    // Display => 3.14
    println!("PI = {:.2}", v);
    // Debug => 3.14
    println!("PI = {:.2?}", v);

    // 宽度 指定输出目标的宽度 长度不够时进行填充和对齐 默认使用空格
    // 以下全部输出 "Hello x    !"
    // 为"x"后面填充空格，补齐宽度5
    println!("Hello {0:5}!", "x");
    // 使用参数5来指定宽度 将x插入到占位符中 并且使用 第1个(从0开始)位置参数为宽度值
    println!("Hello {:1$}!", "x", 5);
    // 使用x作为占位符输出内容，同时使用5作为宽度
    println!("Hello {1:0$}!", 5, "x");
    // 使用有名称的参数作为宽度
    println!("Hello {:width$}!", "x", width = 5);
    //-----------------------------------

    // 使用参数5为参数x指定宽度，同时在结尾输出参数5 => Hello x    !5
    println!("Hello {:1$}!{}", "x", 5);

    // 补齐
    // 以下全部都会补齐5个字符的长度
    // 左对齐 => Hello x    !
    println!("Hello {:<5}!", "x");
    // 右对齐 => Hello     x!
    println!("Hello {:>5}!", "x");
    // 居中对齐 => Hello   x  !
    println!("Hello {:^5}!", "x");

    // 对齐并使用指定符号填充 => Hello x&&&&!
    // 指定符号填充的前提条件是必须有对齐字符
    println!("Hello {:&<5}!", "x");

    // 浮点数精度控制
    let v = std::f64::consts::PI;
    // 保留小数点后两位 => 3.14
    println!("{:.2}", v);
    // 带符号保留小数点后两位 => +3.14
    println!("{:+.2}", v);
    // 不带小数 => 3
    println!("{:.0}", v);
    // 通过参数来设定精度 => 3.1416，相当于{:.4}
    println!("{:.1$}", v, 4);

    let s = "hi我是Sunface孙飞";
    // 保留字符串前三个字符 => hi我
    println!("{:.3}", s);
    // {:.*}接收两个参数，第一个是精度，第二个是被格式化的值 => Hello abc!
    println!("Hello {:.*}!", 3, "abcdefg");

    // 进制
    // 二进制 => 0b11011!
    println!("{:#b}!", 27);
    // 八进制 => 0o33!
    println!("{:#o}!", 27);
    // 十进制 => 27!
    println!("{}!", 27);
    // 小写十六进制 => 0x1b!
    println!("{:#x}!", 27);
    // 大写十六进制 => 0x1B!
    println!("{:#X}!", 27);

    // 不带前缀的十六进制 => 1b!
    println!("{:x}!", 27);

    // 使用0填充二进制，宽度为10 => 0b00011011!
    println!("{:#010b}!", 27);

    // 指数
    println!("{:2e}", 1000000000); // => 1e9
    println!("{:2E}", 1000000000); // => 1E9

    // 指针地址
    let v = vec![1, 2, 3];
    println!("v ptr is {:p}", v.as_ptr()); // => 0x600002324050

    // 转义字符
    // "{{" 转义为 '{'   "}}" 转义为 '}'   "\"" 转义为 '"'
    // => Hello "{World}"
    let world = "world";
    println!("转义字符 Hello \"{world}\" ");
}

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

impl Default for User {
    fn default() -> Self {
        User {
            name: "王磊".to_string(),
            age: 36,
        }
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let formatted = write!(f, "name is {} age is {}", self.name, self.age);

        formatted
    }
}
