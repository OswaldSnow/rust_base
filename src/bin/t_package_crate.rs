fn main() {
    /*
    Package 和 crate
    */

    // Package 是一个 项目工程
    // 包 crate 是一个编译单元


    // 相关方式代码写在了 lib.rs 和 main.rs 中

    // pub 意味着可见性无任何限制
    // pub(crate) 表示在当前包可见
    // pub(self) 在当前模块可见
    // pub(super) 在父模块可见
    // pub(in <path>) 表示在某个路径代表的模块中可见，其中 path 必须是父模块或者祖先模块

}