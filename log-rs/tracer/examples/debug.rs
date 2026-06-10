#[derive(Debug)]
struct MyStruct {
    field: &'static str,
}

fn main() {
    let my_struct = MyStruct {
        field: "Hello world!",
    };

    // `my_struct` 将使用 Debug 的形式输出
    event!(Level::TRACE, greeting = ?my_struct);
    // 等价于:
    event!(Level::TRACE, greeting = tracing::field::debug(&my_struct));

    // 下面代码将报错, my_struct 没有实现 Display
    // event!(Level::TRACE, greeting = my_struct);

    // 日志输出 -> TRACE test_tracing: greeting=MyStruct { field: "Hello world!" }

    // ---

    // `my_struct.field` 将使用 `fmt::Display` 的格式化形式输出
    event!(Level::TRACE, greeting = %my_struct.field);
    // 等价于:
    event!(
        Level::TRACE,
        greeting = tracing::field::display(&my_struct.field)
    );

    // 作为对比，大家可以看下 Debug 和正常的字段输出长什么样
    event!(Level::TRACE, greeting = ?my_struct.field);
    event!(Level::TRACE, greeting = my_struct.field);

    // 下面代码将报错, my_struct 没有实现 Display
    // event!(Level::TRACE, greeting = %my_struct);
}
