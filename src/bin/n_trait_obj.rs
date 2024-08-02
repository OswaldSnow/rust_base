fn main() {
    /*
    特征对象
     */
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    // 特征对象的限制
    // 不是所有特征都可以使用特征对象 对象安全的特征才能使用特征对象
    // 一个特征的所有方法都必须满足
    // 1、返回值不是 Self
    // 2、没有泛型参数
}

pub trait Draw {
    fn draw(&self);
}

#[allow(unused)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
#[allow(unused)]
impl Draw for Button {
    fn draw(&self) {
        // 绘制按钮的代码
        println!("draw Button")
    }
}
#[allow(unused)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 绘制SelectBox的代码
        println!("draw SelectBox")
    }
}

// 集合中存放的是实现了 Draw 特征的类型
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// 绘画出集合中的组件
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// 如果使用泛型的方式
// T 要么都为 A 类型 要么 都为 B 类型
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }
//
// impl<T> Screen<T>
//     where T: Draw {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

// fn draw1(x: Box<dyn Draw>) {
//     // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
//     x.draw();
// }
//
//
// fn draw2(x: &impl Draw){
//     x.draw();
// }
