https://beatai.org/rust-course/basic/match-pattern/all-patterns

## @绑定

@（读作 at）运算符允许为一个字段绑定另外一个变量。下面例子中，我们希望测试 Message::Hello 的 id 字段是否位于 3..=7 范围内，同时也希望能将其值绑定到 id_variable 变量中以便此分支中相关的代码可以使用它。我们可以将 id_variable 命名为 id，与字段同名，不过出于示例的目的这里选择了不同的名称。
